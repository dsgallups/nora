use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Edge {
    sender: Entity,
    receiver: Entity,
}
pub const LINE_MESH_W: f32 = 1.;
pub const LINE_MESH_H: f32 = 1.;

impl Edge {
    pub fn new(sender: Entity, receiver: Entity) -> Self {
        Self { sender, receiver }
    }
    pub fn sender(&self) -> Entity {
        self.sender
    }
    pub fn receiver(&self) -> Entity {
        self.receiver
    }
}

pub(super) fn plugin(app: &mut App) {
    //todo
}
