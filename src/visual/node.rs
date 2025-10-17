use bevy::{platform::collections::HashMap, prelude::*};
use uuid::Uuid;

use crate::visual::Edge;

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

const MIN_DISTANCE: f32 = 80.;

struct NodeLocationMap {
    inner: HashMap<Entity, Vec3>,
}

impl NodeLocationMap {
    fn set_current(&mut self, entity: Entity, loc: Vec3) {
        self.inner.insert(entity, loc);
    }

    fn space(&mut self, node_1: Entity, node_2: Entity) {
        let mut node_1_trns = *self.inner.get(&node_1).unwrap();
        let node_2_trns = *self.inner.get(&node_2).unwrap();
        //
        let distance = node_1_trns.distance(node_2_trns);
        if distance < MIN_DISTANCE {
            let additional_distance = MIN_DISTANCE - distance;

            let add = (node_1_trns - node_2_trns).normalize_or_zero() * additional_distance;

            node_1_trns += add;

            //todo
        }
        let trns = self.inner.get_mut(&node_1).unwrap();
        *trns = node_1_trns;
    }

    fn iter(&self) -> impl Iterator<Item = (&Entity, &Vec3)> {
        self.inner.iter()
    }
}
impl Default for NodeLocationMap {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

fn space_out_nodes(
    nodes: Query<(Entity, &Edges)>,
    mut transforms: Query<&mut Transform, (With<Edges>, Without<Edge>)>,
    edges: Query<&Edge>,
    mut map: Local<NodeLocationMap>,
) {
    for (entity, _) in &nodes {
        let current_translation = transforms.get(entity).unwrap().translation;
        map.set_current(entity, current_translation);
    }

    for (_, node_edges) in &nodes {
        for edge in &node_edges.0 {
            let edge = edges.get(*edge).unwrap();

            map.space(edge.sender(), edge.receiver());
        }
    }
    for (entity, translation) in map.iter() {
        let mut transform = transforms.get_mut(*entity).unwrap();
        transform.translation = *translation;
    }
}
