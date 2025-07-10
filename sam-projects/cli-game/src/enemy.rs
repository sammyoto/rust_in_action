use crate::entity::{Entity, EntityBehavior};

pub struct Enemy {
    pub entity: Entity,
}

impl EntityBehavior for Enemy {
    fn entity(&self) -> &Entity {
        &self.entity
    }
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}