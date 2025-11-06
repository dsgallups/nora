use std::sync::{Arc, RwLock};

use crate::prelude::*;

pub type NeuronPropsAlias = PolyProps<Arc<RwLock<SimpleNeuron>>>;
