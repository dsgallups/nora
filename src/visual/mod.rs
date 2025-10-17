mod line;
pub use line::*;

mod node;
pub use node::*;

use bevy::{asset::uuid::Uuid, platform::collections::HashMap, prelude::*};

use crate::{AppState, brain::Nora};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((line::plugin, node::plugin));
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

    let mut map = HashMap::new();

    for neuron in brain.neurons() {
        let neuron_entity = commands
            .spawn((
                Nid(neuron.id()),
                Mesh2d(circle.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(x, 0., 0.),
            ))
            .id();

        map.insert(neuron.id(), neuron_entity);

        commands.spawn((
            Text2d::new(neuron.name()),
            TextColor(Color::BLACK),
            ChildOf(neuron_entity),
        ));
        x += 70.;
    }

    for neuron in brain.neurons() {
        let neuron_e = map.get(&neuron.id()).unwrap();

        for dendrite in neuron.dendrites() {
            let connected_to = dendrite.connected_to();
            let receives_from = map.get(&connected_to).unwrap();

            commands.spawn((
                Line::new(*receives_from, *neuron_e),
                Mesh2d(meshes.add(Rectangle::new(LINE_MESH_W, LINE_MESH_H))),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::default(),
            ));
        }
    }
}
