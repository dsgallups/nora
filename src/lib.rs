pub mod axon;
pub mod dendrite;
pub mod neuron;

pub struct Brain {}

pub mod prelude {
    pub use crate::dendrite::*;
    pub use crate::neuron::*;
}
