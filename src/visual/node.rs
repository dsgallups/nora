use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct Nid(pub Uuid);

pub(super) fn plugin(app: &mut App) {
    //todo
}
