use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseScrollUnit},
    prelude::*,
};

#[derive(Component, Reflect)]
pub struct StopCameraMovement;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(Update, update_zoom);
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0., 0., 5.)));
}

fn update_zoom(input: Res<AccumulatedMouseScroll>, camera: Query<&mut Transform, With<Camera2d>>) {
    let acc = match input.unit {
        MouseScrollUnit::Line => input.delta,
        MouseScrollUnit::Pixel => input.delta,
    };
    if acc == Vec2::ZERO {
        return;
    }
    for mut transform in camera {
        if acc.y < 0. {
            transform.scale += Vec3::splat(0.2);
        } else {
            transform.scale -= Vec3::splat(0.2);
        }
    }
}
