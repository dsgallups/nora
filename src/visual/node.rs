use bevy::{color::palettes::tailwind::RED_400, platform::collections::HashMap, prelude::*};
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
    app.add_systems(Update, (space_out_nodes, update_node_colors));

    app.add_message::<NodeUpdates>();
}

#[derive(Message)]
pub struct NodeUpdates {
    map: HashMap<Uuid, i32>,
}
impl NodeUpdates {
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

pub const NODE_RADIUS: f32 = 20.;

const MIN_DISTANCE: f32 = 140.;

struct LineInfo {
    n1: Uuid,
    n2: Uuid,
    from: Vec2,
    to: Vec2,
    length: f32,
}

struct NodeLocationMap {
    inner: HashMap<Uuid, (Vec2, Entity)>,
    lines: Vec<LineInfo>,
}

impl NodeLocationMap {
    fn set_current(&mut self, node_id: Uuid, loc: Vec2, entity: Entity) {
        /*
        if there is a value in inner, and the entity was just added,
        do not set the value.
        if there was no value in inner, set the value.
        */
        let (old_loc, old_entity) = self.inner.entry(node_id).or_insert((loc, entity));
        if entity == *old_entity {
            *old_loc = loc;
        } else {
            *old_entity = entity;
        }

        //self.inner.insert(node_id, (loc, e));
    }

    fn set_edges<'a>(
        &mut self,
        edges: impl IntoIterator<Item = &'a Edge>,
        map: &HashMap<Entity, Uuid>,
    ) {
        self.lines.clear();
        for edge in edges {
            let sender = map.get(&edge.sender()).unwrap();
            let (sender_loc, _) = self.inner.get(sender).unwrap();
            let recv = map.get(&edge.receiver()).unwrap();
            let (recv_loc, _) = self.inner.get(recv).unwrap();
            let length = (*recv_loc - *sender_loc).length();
            self.lines.push(LineInfo {
                n1: *sender,
                n2: *recv,
                from: *sender_loc,
                to: *recv_loc,
                length,
            });
        }
    }

    fn space(&mut self) {
        let mut vect = self.inner.iter().map(|v| (*v.0, v.1.0)).collect::<Vec<_>>();
        let len = vect.len();

        let mut neighbors: Vec<Vec2> = Vec::with_capacity(len / 2);

        for i in 0..len {
            neighbors.clear();
            let (node, node_loc) = vect[i];

            let (mut closest_neighbor_loc, mut diff_squared) = (node_loc, f32::MAX);

            for (neighbor, neighbor_loc) in vect.iter() {
                if node == *neighbor {
                    continue;
                }

                let ds = node_loc.distance_squared(*neighbor_loc);

                if ds < diff_squared {
                    closest_neighbor_loc = *neighbor_loc;
                    diff_squared = ds;
                }
            }
            let distance = node_loc.distance(closest_neighbor_loc);
            if distance < MIN_DISTANCE {
                let add = (node_loc - closest_neighbor_loc).normalize_or_zero();
                vect[i].1 += add;
                continue;
            }

            for LineInfo {
                n1,
                n2,
                from: a,
                to: b,
                length,
            } in self.lines.iter()
            {
                if &node == n1 || &node == n2 {
                    continue;
                }
                let ab = *b - *a;
                let ac = node_loc - *a;
                let t = ac.dot(ab) / ab.dot(ab);
                if !(0_f32..=*length).contains(&t) {
                    continue;
                }
                let closest = *a + t * ab;
                let diff = closest - node_loc;
                let dist_sq = diff.length_squared();
                if dist_sq < NODE_RADIUS * 6. && dist_sq < diff_squared {
                    closest_neighbor_loc = closest;
                }
                let distance = node_loc.distance(closest_neighbor_loc);
                if distance < MIN_DISTANCE {
                    let add = (node_loc - closest_neighbor_loc).normalize_or_zero();
                    vect[i].1 += add;
                }
            }
        }
        for (e, loc) in vect {
            let v = self.inner.get_mut(&e).unwrap();
            v.0 = loc;
        }
    }

    fn iter(&self) -> impl Iterator<Item = (&Entity, &Vec2)> {
        self.inner.values().map(|(e, v)| (v, e))
    }
}
impl Default for NodeLocationMap {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
            lines: Vec::new(),
        }
    }
}

fn space_out_nodes(
    nodes: Query<(Entity, Ref<Nid>, &Edges)>,
    mut transforms: Query<&mut Transform, (With<Edges>, Without<Edge>)>,
    edges: Query<&Edge>,
    mut map: Local<NodeLocationMap>,
    mut rev_map: Local<HashMap<Entity, Uuid>>,
) {
    rev_map.clear();
    for (entity, id, _) in &nodes {
        let Ok(transform) = transforms.get(entity) else {
            continue;
        };
        map.set_current(id.0, transform.translation.xy(), entity);
        rev_map.insert(entity, id.0);
    }
    map.set_edges(edges.iter(), &rev_map);

    map.space();

    for (entity, translation) in map.iter() {
        let Ok(mut transform) = transforms.get_mut(*entity) else {
            continue;
        };
        transform.translation.x = translation.x;
        transform.translation.y = translation.y;
    }
}

fn update_node_colors(
    mut reader: MessageReader<NodeUpdates>,
    nodes: Query<(&Nid, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for update in reader.read() {
        for (node, material_handle) in nodes {
            let Some(material) = materials.get_mut(&material_handle.0) else {
                continue;
            };
            let val = update.map.get(&node.0).copied().unwrap_or_default();

            if val > 0 {
                material.color = RED_400.into();
            } else {
                material.color = Color::WHITE;
            }
        }
        //todo
    }
}
