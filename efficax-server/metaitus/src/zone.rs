use std::collections::HashMap;

use cgmath::{Vector2, Zero};

use efficax_utils::id_gen::IdGen;

use super::{physics::collider::PhysicsCollider, entity::MetaitusEntity};

pub struct MetaitusZone {
    // entity_id, entity
    entities: HashMap<u64, MetaitusEntity>,
    // cell_index, entities
    cells: HashMap<u32, Vec<u64>>,
    // cell_index, statics
    statics: HashMap<u32, Vec<PhysicsCollider>>,
    // static_id, cell_indicies
    static_cells: HashMap<u64, Vec<u32>>,
    
    entity_id_gen: IdGen,
    static_id_gen: IdGen
}

impl MetaitusZone {
    const DIMENSION_LENGTH: u32 = 131072;
    const CELL_SIZE: u32 = 16;
    const BELOW_CELL_SIZE: f32 = MetaitusZone::CELL_SIZE as f32 * 0.99;
    const HALF_CELL_SIZE: u32 = Self::CELL_SIZE / 2;
    const DIMENSION_CELL_LENGTH: u32 = Self::DIMENSION_LENGTH / Self::CELL_SIZE;
    const HALF_DIMENSION_CELL_LENGTH: u32 = Self::DIMENSION_CELL_LENGTH / 2;

    pub fn new() -> Self {
        MetaitusZone {
            entities: HashMap::new(),

            cells: HashMap::new(),

            statics: HashMap::new(),

            static_cells: HashMap::new(),

            entity_id_gen: IdGen::new(0),
            static_id_gen: IdGen::new(1)
        }
    }

    pub fn tick(&mut self, tick_id: u64, delta_time: f32) {

        // near_statics, repulsion_vector
        let cached_entity_data = self.calculate_entity_data();

        for (i, entity) in self.entities.values_mut().enumerate() {
            let (near_statics, repulsion_vector) = &cached_entity_data[i];
            entity.add_force(*repulsion_vector, delta_time);
            let moved_xy = entity.tick(tick_id, delta_time, &near_statics);
            if moved_xy {
                let last_cell_index = entity.current_cell_index;
                entity.current_cell_index = Self::get_index_at_pos(entity.pos);
                if last_cell_index != entity.current_cell_index {
                    let last_cell = self.cells.get_mut(&last_cell_index).unwrap();
                    last_cell.retain(|&id| id != entity.id);
                    if last_cell.len() == 0 {
                        self.cells.remove(&last_cell_index);
                    }
                    let current_cell = self.cells.entry(entity.current_cell_index).or_insert_with(Vec::new);
                    current_cell.push(entity.id);
                }
            }
        }
    }

    fn calculate_entity_data(&self) -> Vec<(Vec<PhysicsCollider>, Vector2<f32>)> {
        let mut cached_entity_data = Vec::new();

        for entity in self.entities.values() {
            let cell_indicies = self.get_cell_and_surrounding_indicies(entity.pos);
            let mut near_statics = Vec::new();
            let mut repulsion_vector = Vector2::zero();
            for cell_index in &cell_indicies {
                // find nearby statics
                if let Some(cell_statics) = self.statics.get(cell_index) {
                    for &cell_static in cell_statics {
                        near_statics.push(cell_static);
                    }
                }

                // find nearby entities to repulse
                if let Some(entities) = self.cells.get(cell_index) {
                    for entity_id in entities {
                        if let Some(near_entity) = self.entities.get(entity_id) {
                            if near_entity.id == entity.id || !near_entity.has_repulsion_radius {
                                continue
                            }
                            repulsion_vector += Self::get_repulsion_force(entity, near_entity);
                        }
                    }
                }
            }
            cached_entity_data.push((near_statics, repulsion_vector));
        }

        cached_entity_data
    }

    fn get_repulsion_force(entity: &MetaitusEntity, other: &MetaitusEntity) -> Vector2<f32> {
        let diff_x = other.pos.x - entity.pos.x;
        let diff_y = other.pos.y - entity.pos.y;
        let sqr_distance = (diff_x * diff_x) + (diff_y * diff_y);
        let center_distance = other.repulsion_radius + entity.repulsion_radius;
        if center_distance * center_distance > sqr_distance {
            if !sqr_distance.is_zero() {
                let mut repulsion_mag = (other.repulsion + entity.repulsion) * (1.0 / sqr_distance);
                repulsion_mag = repulsion_mag.clamp(-entity.max_repulsion_mag, entity.max_repulsion_mag);
                let distance = sqr_distance.sqrt();

                return -Vector2::new(diff_x / distance, diff_y / distance) * repulsion_mag;
            }
        }
        Vector2::zero()
    }

    fn get_cell_and_surrounding_indicies(&self, pos: Vector2<f32>) -> Vec<u32> {
        let mut cell_indicies = Vec::with_capacity(9);
        let int_coords = Self::get_int_coords_at_pos(pos);
        let index = Self::get_index_at_int_coords(int_coords);
        cell_indicies.push(index);
        cell_indicies.push(index - Self::DIMENSION_CELL_LENGTH);
        cell_indicies.push(index + Self::DIMENSION_CELL_LENGTH);
        cell_indicies.push(index + 1);
        cell_indicies.push(index - 1);
        cell_indicies.push((index - Self::DIMENSION_CELL_LENGTH) + 1);
        cell_indicies.push((index - Self::DIMENSION_CELL_LENGTH) - 1);
        cell_indicies.push((index + Self::DIMENSION_CELL_LENGTH) + 1);
        cell_indicies.push((index + Self::DIMENSION_CELL_LENGTH) - 1);
        cell_indicies
    }
}

impl MetaitusZone {
    fn get_index_at_pos(pos: Vector2<f32>) -> u32 {
        Self::get_index_at_int_coords(Self::get_int_coords_at_pos(pos))
    }
    fn get_index_at_int_coords(coords: Vector2<u32>) -> u32 {
        (Self::DIMENSION_CELL_LENGTH * coords.y) + coords.x
    }

    fn get_int_coords_at_pos(pos: Vector2<f32>) -> Vector2<u32> {
        Vector2::new(Self::get_int_coord(pos.x), Self::get_int_coord(pos.y))
    }
    fn get_int_coord(dimension: f32) -> u32 {
        (dimension as u32 / Self::CELL_SIZE) + Self::HALF_DIMENSION_CELL_LENGTH
    }

    fn get_cell_center_pos(index: u32) -> Vector2<f32> {
        let cell_pos = Self::get_cell_pos(index);
        Vector2::new(cell_pos.x + Self::HALF_CELL_SIZE as f32, cell_pos.y + Self::HALF_CELL_SIZE as f32)
    }
    fn get_cell_pos(index: u32) -> Vector2<f32> {
        let x = ((index as f32 % Self::DIMENSION_CELL_LENGTH as f32) - Self::HALF_DIMENSION_CELL_LENGTH as f32) * Self::CELL_SIZE as f32;
        let y = ((index as f32 / Self::DIMENSION_CELL_LENGTH as f32) - Self::HALF_DIMENSION_CELL_LENGTH as f32) * Self::CELL_SIZE as f32;
        return Vector2::new(x, y)
    }
}

impl MetaitusZone {
    pub fn add_cell_static(&mut self, collider: PhysicsCollider) {
        // assign static a static id if needed
        let collider = if collider.is_static() {
            collider
        } else {
            collider.copy_with_id(self.static_id_gen.get())
        };

        let min_int_coords = Self::get_int_coords_at_pos(collider.min);
        let max_int_coords = Self::get_int_coords_at_pos(collider.max);
        for y in min_int_coords.y..=max_int_coords.y {
            for x in min_int_coords.x..=max_int_coords.x {
                let index = Self::get_index_at_int_coords(Vector2::new(x, y));

                if let Some(statics) = self.statics.get_mut(&index) {
                    statics.push(collider);
                } else {
                    self.statics.insert(index, vec![collider]);
                }

                if let Some(cell_indicies) = self.static_cells.get_mut(&collider.id) {
                    cell_indicies.push(index);
                } else {
                    self.static_cells.insert(collider.id, vec![index]);
                }
            }
        }
    }
    pub fn remove_cell_static(&mut self, id: u64) {
        if let Some(cell_indicies) = self.static_cells.get_mut(&id) {
            for index in cell_indicies.iter() {
                if let Some(statics) = self.statics.get_mut(&index) {
                    statics.retain(|static_collider| static_collider.id != id);
                }
            }
            self.static_cells.remove(&id);
        }
    }
}