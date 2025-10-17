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
    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    let mut neuron_1 = Neuron::new("N1");
    let mut neuron_2 = Neuron::new("N2");
    let mut neuron_3 = Neuron::new("N3");
    //let junction = ActionPotential::default();
    neuron_3.tx_to(&mut neuron_2);
    neuron_2.tx_to(&mut neuron_1);
    neuron_1.tx_to(&mut neuron_3);
    _ = neuron_2.fire(1);

    let mut brain = Brain::new("Brain");

    brain.add(neuron_1);
    brain.add(neuron_2);
    brain.add(neuron_3);
    commands.insert_resource(Nora::new(brain));
}
