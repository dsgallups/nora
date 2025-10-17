use bevy::prelude::*;

use crate::{AppState, brain::Nora};

pub(super) fn plugin(app: &mut App) {
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
}
