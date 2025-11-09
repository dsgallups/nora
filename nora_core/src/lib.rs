#![allow(clippy::derivable_impls)]

pub mod topology;
pub mod utils;

pub mod prelude {
    //pub use crate::channel::*;
    pub use crate::topology::*;
    pub use crate::utils::*;

    pub use trotcast::prelude::*;
}

//pub mod neat;
