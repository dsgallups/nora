use bevy::prelude::*;
use nora_core::prelude::*;

use crate::{
    AppState,
    visual::{EdgeUpdates, NodeUpdates},
};

#[derive(Clone, Copy)]
enum BrainState {
    Dendrite,
    Axon,
}

#[derive(Resource)]
pub struct Nora {
    frame_state: BrainState,
    brain: Brain,
}

impl Nora {
    pub fn new(brain: Brain) -> Self {
        Self {
            frame_state: BrainState::Dendrite,
            brain,
        }
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
    app.add_observer(update_state).add_observer(update_brain);
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
    commands.trigger(SetLoading);
}
#[derive(Event)]
struct SetLoading;

fn update_state(_: On<SetLoading>, mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Loading);
}

#[derive(Event)]
pub struct UpdateBrain;

fn update_brain(
    _: On<UpdateBrain>,
    mut nora: ResMut<Nora>,
    mut edge_updates: MessageWriter<EdgeUpdates>,
    mut node_updates: MessageWriter<NodeUpdates>,
) {
    match nora.frame_state {
        BrainState::Dendrite => {
            node_updates.write(NodeUpdates::empty());
            edge_updates.write(EdgeUpdates::set(
                nora.brain_mut()
                    .update_dendrites()
                    .map(|msg| (msg.dendrite_id, msg.current_potential)),
            ));
            nora.frame_state = BrainState::Axon;
        }
        BrainState::Axon => {
            edge_updates.write(EdgeUpdates::empty());

            node_updates.write(NodeUpdates::set(
                nora.brain_mut()
                    .update_axons()
                    .map(|msg| (msg.id, msg.discharge)),
            ));

            nora.frame_state = BrainState::Dendrite;
        }
    }
}
