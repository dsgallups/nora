use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::prelude::Dendrite;

pub struct Axon {
    id: Uuid,
    inner: Arc<Mutex<AxonInner>>,
    connections: Vec<Dendrite>,
}

impl Default for Axon {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: Arc::new(Mutex::new(AxonInner::default())),
            connections: Vec::new(),
        }
    }
}

struct AxonInner;

impl Default for AxonInner {
    fn default() -> Self {
        Self
    }
}
