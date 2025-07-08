use crate::entity::{Entity, EntityBehavior};

pub struct Player {
    pub entity: Entity,
}

impl EntityBehavior for Player {
    fn entity(&self) -> &Entity {
        &self.entity
    }
    fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }
}