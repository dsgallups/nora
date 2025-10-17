use bevy::prelude::*;

use crate::visual::Edges;

#[derive(Component, Reflect)]
pub struct Edge {
    sender: Entity,
    receiver: Entity,
}
pub const LINE_MESH_X: f32 = 1.;
pub const LINE_MESH_Y: f32 = 2.;

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
    app.add_systems(Update, update_edges);
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
        let normal = val.normalize_or_zero();
        if length > 0. {
            transform.scale.x = length;
        }
        transform.translation = sender_trns + (normal * length * 0.5);

        let angle = normal.y.atan2(normal.x);

        transform.rotation = Quat::from_rotation_z(angle);
        //transform.rotation = Quat::from_rotation_arc_2d(Vec2::new(-50., -50.), Vec2::new(50., 50.));

        //todo
    }
}
