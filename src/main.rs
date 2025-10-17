pub mod brain;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1920, 1080),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(brain::plugin);

    app.run();
}
