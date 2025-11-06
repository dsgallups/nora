//pub mod burn_net;

pub mod core;

pub mod simple_net;

pub mod topology;

mod test_utils;

pub mod prelude {
    pub use super::core::{
        activation::{Bias, Exponent},
        input::NeuronInput,
        //neuron::PolyNeuronInner,
        neuron_type::{NeuronProps, NeuronType, PropsType},
    };
    pub use super::simple_net::{
        input::NeuronInputAlias, network::SimplePolyNetwork, neuron::SimpleNeuron,
        neuron_type::NeuronPropsAlias,
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
