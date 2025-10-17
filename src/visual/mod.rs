mod edge;
pub use edge::*;

mod node;
pub use node::*;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::brain::Nora;

const NODE_LAYER: f32 = 1.;
const EDGE_LAYER: f32 = 0.;

#[derive(Component)]
pub struct GraphComponent;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((edge::plugin, node::plugin));
    app.add_systems(Startup, setup);
    app.add_observer(respawn_visualization);
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 5.)));
}
#[derive(Event)]
pub struct RespawnVisualization;

fn respawn_visualization(
    _: On<RespawnVisualization>,
    mut commands: Commands,
    graph_components: Query<Entity, With<GraphComponent>>,
    nora: Res<Nora>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in graph_components {
        commands.entity(entity).despawn();
    }

    let brain = nora.brain();

    let circle = meshes.add(Circle::new(NODE_RADIUS));

    let mut x = 0.;
    let mut y = -20.;

    let mut map = HashMap::new();

    for neuron in brain.neurons() {
        let neuron_entity = commands
            .spawn((
                GraphComponent,
                Nid(neuron.id()),
                Mesh2d(circle.clone()),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_xyz(x, y, NODE_LAYER),
            ))
            .id();

        map.insert(neuron.id(), neuron_entity);

        commands.spawn((
            Text2d::new(neuron.name()),
            TextColor(Color::BLACK),
            ChildOf(neuron_entity),
        ));
        x += 12.;
        y *= -1.;
    }

    for neuron in brain.neurons() {
        let neuron_e = map.get(&neuron.id()).unwrap();

        let mut lines = Vec::new();

        for dendrite in neuron.dendrites() {
            let connected_to = dendrite.connected_to();
            let receives_from = map.get(&connected_to).unwrap();

            let line = commands
                .spawn((
                    GraphComponent,
                    Edge::new(dendrite.id(), *receives_from, *neuron_e),
                    Mesh2d(meshes.add(Rectangle::new(LINE_MESH_X, LINE_MESH_Y))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0., 0., EDGE_LAYER),
                ))
                .id();
            lines.push(line);
        }
        commands.entity(*neuron_e).insert(Edges::new(lines));
    }
}
