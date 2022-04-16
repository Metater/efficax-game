use super::entity::MetaitusEntity;

pub struct MetaitusCell {
    pub index: u32,
    pub entities: Vec<MetaitusEntity>
}

impl MetaitusCell {
    pub fn new(index: u32) -> MetaitusCell {
        MetaitusCell {
            index,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: MetaitusEntity) {
        self.entities.push(entity);
    }
    pub fn remove_entity(&mut self, id: u32) -> Option<MetaitusEntity> {
        if let Some(index) = self.entities.iter().position(|entity| entity.id == id) {
            return Some(self.entities.remove(index));
        }
        return None
    }
}