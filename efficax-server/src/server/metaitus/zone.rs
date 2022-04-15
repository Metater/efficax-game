use std::collections::HashMap;

use cgmath::Vector2;

use super::cell::MetaitusCell;

pub struct MetaitusZone {
    cells: HashMap<u32, MetaitusCell>
}

impl MetaitusZone {
    const DIMENSION_LENGTH: u32 = 131072;
    const CELL_SIZE: u32 = 16;
    const HALF_CELL_SIZE: u32 = MetaitusZone::CELL_SIZE;
    const DIMENSION_CELL_LENGTH: u32 = MetaitusZone::DIMENSION_LENGTH / MetaitusZone::CELL_SIZE;
    const HALF_DIMENSION_CELL_LENGTH: u32 = MetaitusZone::DIMENSION_CELL_LENGTH / 2;

    pub fn new() -> Self {
        MetaitusZone {
            cells: HashMap::new()
        }
    }

    pub fn get_cell_and_surrounding(&self, pos: Vector2<f32>) -> Vec<u32> {
        let int_coords = MetaitusZone::get_int_coords_at_pos(pos);
        let x = int_coords.x;
        let y = int_coords.y;
        let mut cell_indicies = Vec::new();
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
    pub fn push_cell_index(&self, coord_x: u32, coord_y: u32, cell_indicies: &mut Vec<u32>) {
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