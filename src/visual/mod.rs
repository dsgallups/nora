mod edge;

use bimap::BiMap;
pub use edge::*;

mod node;
pub use node::*;

use bevy::prelude::*;
use uuid::Uuid;

use crate::brain::Nora;

const NODE_LAYER: f32 = 1.;
const EDGE_LAYER: f32 = 0.;

#[derive(Resource, Default)]
pub struct EntityGraphMap {
    entity_map: BiMap<Entity, Uuid>,
}

impl EntityGraphMap {
    pub fn insert(&mut self, entity: Entity, id: Uuid) {
        self.entity_map.insert(entity, id);
    }
    pub fn get_uuid(&self, entity: &Entity) -> Option<&Uuid> {
        self.entity_map.get_by_left(entity)
    }
    pub fn get_entity(&self, uuid: &Uuid) -> Option<&Entity> {
        self.entity_map.get_by_right(uuid)
    }
    fn remove(&mut self, entity: &Entity) {
        self.entity_map.remove_by_left(entity);
    }
}

#[derive(Component)]
pub struct GraphComponent;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EntityGraphMap>();
    app.add_plugins((edge::plugin, node::plugin));
    app.add_systems(PreUpdate, spawn_new_nodes);
    app.add_systems(PostUpdate, (despawn_dead_nodes, despawn_dead_edges).chain());
}

fn spawn_new_nodes(
    mut commands: Commands,
    nora: Res<Nora>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<EntityGraphMap>,
) {
    let brain = nora.brain();

    let circle = meshes.add(Circle::new(NODE_RADIUS));

    let mut x = 0.;
    let mut y = -20.;

    for neuron in brain.neurons() {
        if map.get_entity(&neuron.id()).is_none() {
            let neuron_entity = commands
                .spawn((
                    GraphComponent,
                    Nid(neuron.id()),
                    Mesh2d(circle.clone()),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(x, y, NODE_LAYER),
                ))
                .id();

            map.insert(neuron_entity, neuron.id());

            commands.spawn((
                Text2d::new(neuron.name()),
                TextColor(Color::BLACK),
                ChildOf(neuron_entity),
            ));
            x += 12.;
            y *= -1.;
        }
    }

    for neuron in brain.neurons() {
        let neuron_e = *map.get_entity(&neuron.id()).unwrap();

        let mut new_edges = Vec::new();

        for dendrite in neuron.dendrites() {
            if map.get_entity(&dendrite.id()).is_none() {
                let connected_to = dendrite.connected_to();
                let Some(receives_from) = map.get_entity(&connected_to) else {
                    continue;
                };

                let edge = commands
                    .spawn((
                        GraphComponent,
                        Edge::new(dendrite.id(), *receives_from, neuron_e),
                        Mesh2d(meshes.add(Rectangle::new(LINE_MESH_X, LINE_MESH_Y))),
                        MeshMaterial2d(materials.add(Color::WHITE)),
                        Transform::from_xyz(0., 0., EDGE_LAYER),
                    ))
                    .id();

                map.insert(edge, dendrite.id());

                new_edges.push(edge);
            }
        }
    }

    //
}

fn despawn_dead_nodes(
    mut commands: Commands,
    nodes: Query<(Entity, &Nid)>,
    mut nora: ResMut<Nora>,
    mut map: ResMut<EntityGraphMap>,
) {
    nora.brain_mut().cleanup();
    for (node, id) in nodes {
        if nora.brain().get_neuron(id.0).is_none() {
            map.remove(&node);
            commands.entity(node).despawn();
        }
    }
}

fn despawn_dead_edges(
    mut commands: Commands,
    edges: Query<(Entity, &Edge)>,
    nora: Res<Nora>,
    mut map: ResMut<EntityGraphMap>,
) {
    for (entity, edge) in edges {
        if nora.brain().get_dendrite(edge.id()).is_none() {
            map.remove(&entity);
            commands.entity(entity).despawn();
        }
    }
}
