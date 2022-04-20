use std::collections::HashMap;

use cgmath::{Vector2};

use super::{cell::MetaitusCell, physics::collider::PhysicsCollider, entity::MetaitusEntity};

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
    const HALF_CELL_SIZE: u32 = MetaitusZone::CELL_SIZE;
    const DIMENSION_CELL_LENGTH: u32 = MetaitusZone::DIMENSION_LENGTH / MetaitusZone::CELL_SIZE;
    const HALF_DIMENSION_CELL_LENGTH: u32 = MetaitusZone::DIMENSION_CELL_LENGTH / 2;

    pub fn new() -> Self {
        MetaitusZone {
            entities: HashMap::new(),

            cells: HashMap::new(),

            statics: HashMap::new(),

            entity_id_gen: IdGen::new(0),
            static_id_gen: IdGen::new(0)
        }
    }
}