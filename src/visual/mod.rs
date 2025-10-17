mod edge;
pub use edge::*;

mod node;
pub use node::*;

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{AppState, brain::Nora};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((edge::plugin, node::plugin));
    app.add_systems(Startup, setup);
    app.add_systems(OnEnter(AppState::Loading), spawn_visualization);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_visualization(
    mut commands: Commands,
    nora: Res<Nora>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let brain = nora.brain();

    let circle = meshes.add(Circle::new(20.));
    let color = materials.add(Color::WHITE);

    let mut x = 0.;
    let mut y = -20.;

    let mut map = HashMap::new();

    for neuron in brain.neurons() {
        let neuron_entity = commands
            .spawn((
                Nid(neuron.id()),
                Mesh2d(circle.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(x, y, 0.),
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
                    Edge::new(*receives_from, *neuron_e),
                    Mesh2d(meshes.add(Rectangle::new(LINE_MESH_X, LINE_MESH_Y))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::default(),
                ))
                .id();
            lines.push(line);
        }
        commands.entity(*neuron_e).insert(Edges::new(lines));
    }
}
