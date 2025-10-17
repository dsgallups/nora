use bevy::prelude::*;
use nora_core::prelude::*;

#[derive(Resource)]
pub struct Nora {
    brain: Brain,
}

impl Nora {
    pub fn new(brain: Brain) -> Self {
        Self { brain }
    }
    pub fn brain(&self) -> &Brain {
        &self.brain
    }
    pub fn brain_mut(&mut self) -> &mut Brain {
        &mut self.brain
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Nora::new(Brain::sandbox()));
}
