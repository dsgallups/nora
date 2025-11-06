use std::sync::{Arc, RwLock};

use crate::prelude::*;

pub type NeuronPropsAlias = NeuronProps<Arc<RwLock<SimpleNeuron>>>;
