use bevy::prelude::*;

use crate::{
    brain::Nora,
    visual::{EdgeUpdates, NodeUpdates, RespawnVisualization},
    widgets,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<BrainState>();
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
        children![
            widgets::button("Next Frame", next_brain_frame),
            widgets::button("Add Neuron", add_neuron)
        ],
    )
}
#[derive(Resource, Default, Clone, Copy)]
enum BrainState {
    #[default]
    Dendrite,
    Axon,
}

fn next_brain_frame(
    _: On<Pointer<Click>>,
    mut nora: ResMut<Nora>,
    mut brain_state: ResMut<BrainState>,
    mut edge_updates: MessageWriter<EdgeUpdates>,
    mut node_updates: MessageWriter<NodeUpdates>,
) {
    match *brain_state {
        BrainState::Dendrite => {
            node_updates.write(NodeUpdates::empty());
            edge_updates.write(EdgeUpdates::set(
                nora.brain_mut()
                    .update_dendrites()
                    .map(|msg| (msg.dendrite_id, msg.current_potential)),
            ));
            *brain_state = BrainState::Axon;
        }
        BrainState::Axon => {
            edge_updates.write(EdgeUpdates::empty());

            node_updates.write(NodeUpdates::set(
                nora.brain_mut()
                    .update_axons()
                    .map(|msg| (msg.id, msg.discharge)),
            ));

            *brain_state = BrainState::Dendrite;
        }
    }
}

fn add_neuron(_: On<Pointer<Click>>, mut commands: Commands, mut nora: ResMut<Nora>) {
    nora.brain_mut().add_neuron();
    commands.trigger(RespawnVisualization);
}
