//pub mod burn_net;

pub mod core;

pub mod simple_net;

pub mod topology;

mod test_utils;

pub mod prelude {
    pub use super::core::{
        activation::{Bias, Exponent},
        input::Input,
        //neuron::PolyNeuronInner,
        neuron_type::{NeuronType, PolyProps, PropsType},
    };
    pub use super::simple_net::{
        input::NeuronInput, network::SimplePolyNetwork, neuron::SimpleNeuron,
        neuron_type::NeuronProps,
    };
    pub use super::topology::{
        mutation::{MAX_MUTATIONS, MutationAction, MutationChances},
        network::NetworkTopology,
        neuron::NeuronTopology,
    };
    #[cfg(test)]
    pub(crate) use crate::test_utils::arc;
}

// #[cfg(test)]
// mod tests;
