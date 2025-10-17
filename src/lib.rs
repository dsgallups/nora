#![allow(clippy::default_constructed_unit_structs)]
#![allow(clippy::derivable_impls)]

pub mod glial;
pub mod neuron;

pub mod junction;

pub mod brain;
pub mod communication;
pub mod state_manager;

pub mod prelude {
    pub use crate::brain::*;
    pub use crate::communication::*;
    pub use crate::junction::*;
    pub use crate::neuron::axon::*;
    pub use crate::neuron::dendrite::*;
    pub use crate::neuron::soma::*;
    pub use crate::neuron::*;
    pub use crate::state_manager::*;

    pub use crate::glial::*;
}
