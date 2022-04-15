use super::entity::MetaitusEntity;

pub struct MetaitusCell {
    index: u32,
    entities: Vec<MetaitusEntity>
}

impl MetaitusCell {
    pub fn new(index: u32) -> MetaitusCell {
        MetaitusCell {
            index,
            entities: Vec::new(),
        }
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn add_entity(&mut self, entity: MetaitusEntity) {
        self.entities.push(entity);
    }
    pub fn get_entities(&self) -> &Vec<MetaitusEntity> {
        &self.entities
    }
    pub fn remove_entity(&mut self, id: u32) -> bool {
        if let Some(index) = self.entities.iter().position(|entity_id| entity_id.get_id() == id) {
            self.entities.remove(index);
            return true
        }
        return  false
    }
}