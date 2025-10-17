pub mod brain;
pub mod ui;
pub mod visual;
pub mod widgets;

use bevy::{prelude::*, window::WindowResolution};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Start,
    Loading,
    Run,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1920, 1080),
            ..default()
        }),
        ..default()
    }));
    app.init_state::<AppState>();

    app.add_plugins((brain::plugin, visual::plugin, ui::plugin));

    app.run();
}
