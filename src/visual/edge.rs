use bevy::{platform::collections::HashMap, prelude::*};
use uuid::Uuid;

use crate::visual::Edges;

#[derive(Component, Reflect)]
pub struct Edge {
    id: Uuid,
    sender: Entity,
    receiver: Entity,
}
pub const LINE_MESH_X: f32 = 1.;
pub const LINE_MESH_Y: f32 = 2.;

impl Edge {
    pub fn new(id: Uuid, sender: Entity, receiver: Entity) -> Self {
        Self {
            sender,
            id,
            receiver,
        }
    }
    pub fn sender(&self) -> Entity {
        self.sender
    }
    pub fn receiver(&self) -> Entity {
        self.receiver
    }
}

#[derive(Message)]
pub struct EdgeUpdates {
    map: HashMap<Uuid, i32>,
}
impl EdgeUpdates {
    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn set(values: impl IntoIterator<Item = (Uuid, i32)>) -> Self {
        Self {
            map: values.into_iter().collect(),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_edges);
    app.add_message::<EdgeUpdates>();
}

fn update_edges(
    edges: Query<(&mut Transform, &Edge), Without<Edges>>,
    nodes: Query<&Transform, With<Edges>>,
) {
    for (mut transform, edge) in edges {
        let sender_trns = nodes.get(edge.sender()).unwrap().translation;
        let recv_trns = nodes.get(edge.receiver()).unwrap().translation;

        let val = (recv_trns - sender_trns);
        let length = val.length();
        if length > 0. {
            transform.scale.x = length;
        }
        transform.translation = sender_trns + (val * 0.5);

        let angle = val.y.atan2(val.x);

        transform.rotation = Quat::from_rotation_z(angle);
    }
}
