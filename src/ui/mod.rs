use bevy::prelude::*;

use crate::{brain::Nora, widgets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Flex,
            width: percent(100.),
            height: percent(100.),
            ..default()
        },
        children![actions()],
    ));
}

fn actions() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(10.),
            column_gap: px(10.),
            ..default()
        },
        children![widgets::button("Click", empty)],
    )
}

fn empty(_: On<Pointer<Click>>, mut nora: ResMut<Nora>) {
    info!("Clicked");

    let brain = nora.brain_mut();
    brain.update();
}
