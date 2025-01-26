#![allow(clippy::default_constructed_unit_structs)]
#![allow(clippy::derivable_impls)]

pub mod neuron;

pub struct Brain {}

pub mod prelude {
    pub use crate::neuron::axon::*;
    pub use crate::neuron::dendrite::*;
    pub use crate::neuron::soma::*;
    pub use crate::neuron::*;
}
