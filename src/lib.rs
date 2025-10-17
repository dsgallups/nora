#![allow(clippy::derivable_impls)]

//mod channel;
pub mod topology;

pub mod prelude {
    //pub use crate::channel::*;
    pub use crate::topology::*;

    pub use trotcast::prelude::*;
}

//pub mod neat;
