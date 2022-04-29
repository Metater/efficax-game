use std::collections::HashMap;

use cgmath::{Vector2};

use efficax_utils::id_gen::IdGen;

use super::{physics::collider::PhysicsCollider, entity::MetaitusEntity};

pub struct MetaitusZone {
    // entity_id, entity
    entities: HashMap<u64, MetaitusEntity>,
    // cell_index, entities
    cells: HashMap<u32, Vec<u64>>,
    // cell_index, statics
    statics: HashMap<u32, Vec<PhysicsCollider>>,
    
    entity_id_gen: IdGen,
    static_id_gen: IdGen
}

impl MetaitusZone {
    const DIMENSION_LENGTH: u32 = 131072;
    const CELL_SIZE: u32 = 16;
    const HALF_CELL_SIZE: u32 = Self::CELL_SIZE;
    const DIMENSION_CELL_LENGTH: u32 = Self::DIMENSION_LENGTH / Self::CELL_SIZE;
    const HALF_DIMENSION_CELL_LENGTH: u32 = Self::DIMENSION_CELL_LENGTH / 2;

    pub fn new() -> Self {
        MetaitusZone {
            entities: HashMap::new(),

            cells: HashMap::new(),

            statics: HashMap::new(),

            entity_id_gen: IdGen::new(0),
            static_id_gen: IdGen::new(0)
        }
    }

    fn get_cell_and_surrounding_indicies(&self, pos: Vector2<f32>) -> Vec<u32> {
        let mut cell_indicies = Vec::with_capacity(9);
        let int_coords = Self::get_int_coords_at_pos(pos);
        let index = Self::get_index_at_int_coords(int_coords);
        cell_indicies.push(index);
        cell_indicies.push(index - Self::DIMENSION_CELL_LENGTH); // n
        cell_indicies.push(index + Self::DIMENSION_CELL_LENGTH); // s
        cell_indicies.push(index + 1); // e
        cell_indicies.push(index - 1); // w
        cell_indicies.push((index - Self::DIMENSION_CELL_LENGTH) + 1); // ne
        cell_indicies.push((index - Self::DIMENSION_CELL_LENGTH) - 1); // nw
        cell_indicies.push((index + Self::DIMENSION_CELL_LENGTH) + 1); // se
        cell_indicies.push((index + Self::DIMENSION_CELL_LENGTH) - 1); // sw
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