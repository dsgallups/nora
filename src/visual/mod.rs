use bevy::{asset::uuid::Uuid, prelude::*};

use crate::{AppState, brain::Nora};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(OnEnter(AppState::Loading), spawn_visualization);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
#[derive(Component)]
pub struct Nid(pub Uuid);

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
    for neuron in brain.neurons() {
        info!("here");
        let neuron_entity = commands
            .spawn((
                Nid(neuron.id()),
                Mesh2d(circle.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(x, 0., 0.),
            ))
            .id();

        commands.spawn((
            Text2d::new(neuron.name()),
            TextColor(Color::BLACK),
            ChildOf(neuron_entity),
        ));
        x += 70.;
    }
}
