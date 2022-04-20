use std::collections::HashMap;

use cgmath::{Vector2};

use super::{cell::MetaitusCell, physics::collider::PhysicsCollider, entity::MetaitusEntity};

pub struct MetaitusZone {
    cells: HashMap<u32, MetaitusCell>,
    // find a better way to provide statics, maybe have uniform statics, where one ref is used for like a 1x1 tile
    // then use a list of positions that map to the same static?
    statics: HashMap<u32, Vec<PhysicsCollider>>,

    next_entity_id: u32

    // entity id, entity
    // hashset of populated cell_indicies
    // 
}

impl MetaitusZone {
    const DIMENSION_LENGTH: u32 = 131072;
    const CELL_SIZE: u32 = 16;
    const HALF_CELL_SIZE: u32 = MetaitusZone::CELL_SIZE;
    const DIMENSION_CELL_LENGTH: u32 = MetaitusZone::DIMENSION_LENGTH / MetaitusZone::CELL_SIZE;
    const HALF_DIMENSION_CELL_LENGTH: u32 = MetaitusZone::DIMENSION_CELL_LENGTH / 2;

    pub fn new() -> Self {
        MetaitusZone {
            cells: HashMap::new(),

            statics: HashMap::new(),

            next_entity_id: 0
        }
    }

    pub fn tick(&mut self, tick_id: u64, delta_time: f32) {
        // Terrible time complexity and extra memory used here
        // Optimization: calc repulsion vector and hand over on entity tick

        // CLEAN UP THIS, split into functions

        let mut switched_cell_entities = Vec::new();

        let mut cached_entity_data: Vec<(Vec<PhysicsCollider>, Vec<(Vector2<f32>, f32, f32)>)> = Vec::new();

        for cell in self.cells.values() {
            for entity in &cell.entities {
                let cell_indicies = self.get_cell_and_surrounding(entity.pos);
                let mut near_statics = Vec::new();
                let mut repulsable_entities = Vec::new();
                for &cell_index in &cell_indicies {
                    if let Some(cell_statics) = self.get_cell_statics(cell_index) {
                        near_statics.append(cell_statics.clone().as_mut());
                    }
                    
                    let cell = self.get_existing_cell(cell_index);
                    for cell_entity in &cell.entities {
                        if cell_entity.id == entity.id || !cell_entity.has_repulsion_radius {
                            continue
                        }
                        repulsable_entities.push((cell_entity.pos, cell_entity.repulsion_radius, cell_entity.repulsion));
                    }
                }
                cached_entity_data.push((near_statics, repulsable_entities));
            }
        }

        let mut entity_index = 0;
        for cell in self.cells.values_mut() {
            for entity in &mut cell.entities {
                let (near_statics, repulsable_entities) = &cached_entity_data[entity_index];
                let moved_xy = entity.tick(tick_id, delta_time, near_statics, repulsable_entities);
                if moved_xy {
                    let last_cell_index = entity.current_cell_index;
                    entity.current_cell_index = MetaitusZone::get_index_at_pos(entity.pos);
                    if entity.current_cell_index != last_cell_index {
                        switched_cell_entities.push((last_cell_index, entity.id));
                    }
                }
                else {
                    // not moving
                }
                entity_index += 1;
            }
        }

        for (last_cell_index, switched_entity_id) in switched_cell_entities {
            let last_cell = self.get_cell(last_cell_index);
            if let Some(entity) = last_cell.remove_entity(switched_entity_id) {
                if last_cell.entities.len() == 0 {
                    self.remove_cell(last_cell_index);
                }
                let current_cell = self.get_cell(entity.current_cell_index);
                current_cell.add_entity(entity);
            }
        }
    }

    pub fn get_cell_and_surrounding(&self, pos: Vector2<f32>) -> Vec<u32> {
        let mut cell_indicies = Vec::with_capacity(9);
        let int_coords = MetaitusZone::get_int_coords_at_pos(pos);
        let x = int_coords.x;
        let y = int_coords.y;
        self.push_cell_index(x, y, &mut cell_indicies);
        self.push_cell_index(x, y + 1, &mut cell_indicies);
        self.push_cell_index(x + 1, y + 1, &mut cell_indicies);
        self.push_cell_index(x + 1, y, &mut cell_indicies);
        self.push_cell_index(x + 1, y - 1, &mut cell_indicies);
        self.push_cell_index(x, y - 1, &mut cell_indicies);
        self.push_cell_index(x - 1, y - 1, &mut cell_indicies);
        self.push_cell_index(x - 1, y, &mut cell_indicies);
        self.push_cell_index(x - 1, y + 1, &mut cell_indicies);
        cell_indicies
    }
    fn push_cell_index(&self, coord_x: u32, coord_y: u32, cell_indicies: &mut Vec<u32>) {
        let index = MetaitusZone::get_index_at_int_coords(Vector2::new(coord_x, coord_y));
        if self.cells.contains_key(&index) {
            cell_indicies.push(index);
        }
    }

    pub fn get_cell_at_pos(&mut self, pos: Vector2<f32>) -> &mut MetaitusCell {
        self.get_cell(MetaitusZone::get_index_at_pos(pos))
    }
    pub fn get_cell(&mut self, index: u32) -> &mut MetaitusCell {
        if !self.cells.contains_key(&index) {
            let cell = MetaitusCell::new(index);
            self.cells.insert(index, cell);
        }

        return self.cells.get_mut(&index).unwrap()
    }
    pub fn get_existing_cell(&self, index: u32) -> &MetaitusCell {
        return self.cells.get(&index).unwrap()
    }

    pub fn remove_cell_at_pos(&mut self, pos: Vector2<f32>) {
        self.remove_cell(MetaitusZone::get_index_at_pos(pos))
    }
    pub fn remove_cell(&mut self, index: u32) {
        self.cells.remove(&index);
    }

    pub fn get_index_at_pos(pos: Vector2<f32>) -> u32 {
        MetaitusZone::get_index_at_int_coords(MetaitusZone::get_int_coords_at_pos(pos))
    }
    pub fn get_index_at_int_coords(coords: Vector2<u32>) -> u32 {
        (MetaitusZone::DIMENSION_CELL_LENGTH * coords.y) + coords.x
    }

    fn get_int_coords_at_pos(pos: Vector2<f32>) -> Vector2<u32> {
        Vector2::new(MetaitusZone::get_int_coord(pos.x), MetaitusZone::get_int_coord(pos.y))
    }
    fn get_int_coord(dimension: f32) -> u32 {
        (dimension as u32 / MetaitusZone::CELL_SIZE) + MetaitusZone::HALF_DIMENSION_CELL_LENGTH
    }

    pub fn get_cell_center_pos(index: u32) -> Vector2<f32> {
        let cell_pos = MetaitusZone::get_cell_pos(index);
        Vector2::new(cell_pos.x + MetaitusZone::HALF_CELL_SIZE as f32, cell_pos.y + MetaitusZone::HALF_CELL_SIZE as f32)
    }
    pub fn get_cell_pos(index: u32) -> Vector2<f32> {
        let x = ((index as f32 % MetaitusZone::DIMENSION_CELL_LENGTH as f32) - MetaitusZone::HALF_DIMENSION_CELL_LENGTH as f32) * MetaitusZone::CELL_SIZE as f32;
        let y = ((index as f32 / MetaitusZone::DIMENSION_CELL_LENGTH as f32) - MetaitusZone::HALF_DIMENSION_CELL_LENGTH as f32) * MetaitusZone::CELL_SIZE as f32;
        return Vector2::new(x, y)
    }
}

impl MetaitusZone {
    pub fn spawn_entity(&mut self, pos: Vector2<f32>) -> &mut MetaitusEntity {
        let id = self.get_next_entity_id();
        let current_cell_index = MetaitusZone::get_index_at_pos(pos);

        let mut entity = MetaitusEntity::new(id, pos, current_cell_index);
        entity
        .with_bounds(true, PhysicsCollider::new(0, Vector2::new(-5.0, -3.0), Vector2::new(5.0, 3.0)))
        .with_drag(true, true, 3.0)
        .with_collider(true, PhysicsCollider::new(0, Vector2::new(-0.475, -0.475), Vector2::new(0.475, 0.475)))
        .with_repulsion_radius(true, 0.4, 48.0, 3.0);

        let cell = self.get_cell(current_cell_index);
        let position = cell.entities.len();
        cell.add_entity(entity);
        &mut cell.entities[position]
    }
    pub fn get_entity(&mut self, id: u32) -> Option<&mut MetaitusEntity> {
        for cell in self.cells.values_mut() {
            for entity in &mut cell.entities {
                if entity.id == id {
                    return Some(entity)
                }
            }
        }
        return None
    }
    pub fn despawn_entity(&mut self, id: u32) -> Option<MetaitusEntity> {
        for cell in self.cells.values_mut() {
            if let Some(entity) = cell.remove_entity(id) {
                return Some(entity);
            }
        }
        return None;
    }
    fn get_next_entity_id(&mut self) -> u32 {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        id
    }
}

impl MetaitusZone {
    pub fn add_cell_static(&mut self, index: u32, collider: PhysicsCollider) {
        if !self.statics.contains_key(&index) {
            self.statics.insert(index, vec![collider]);
        }
        else if let Some(cell_statics) = self.statics.get_mut(&index) {
            cell_statics.push(collider);
        }
    }
    pub fn get_cell_statics(&self, index: u32) -> Option<&Vec<PhysicsCollider>> {
        self.statics.get(&index)
    }
    pub fn remove_cell_static(&mut self, index: u32, id: u64) -> bool {
        if let Some(cell_statics) = self.statics.get_mut(&index) {
            if let Some(index) = cell_statics.iter().position(|collider| collider.id == id) {
                cell_statics.remove(index);
                return true
            }
        }
        return  false
    }
}