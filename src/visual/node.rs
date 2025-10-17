use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Reflect)]
pub struct Nid(pub Uuid);

#[derive(Component, Reflect)]
pub struct Edges(Vec<Entity>);
impl Edges {
    pub fn new(lines: Vec<Entity>) -> Self {
        Self(lines)
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, space_out_nodes);
}

fn space_out_nodes(nodes: Query<&Edges>, transforms: Query<&mut Transform, With<Edges>>) {
    for edges in nodes {
        todo!()
    }
}
