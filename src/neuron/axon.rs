use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::prelude::Dendrite;

pub struct Axon {
    id: AxonId,
    inner: Arc<RwLock<AxonInner>>,
    connections: Vec<Dendrite>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxonId(Uuid);

impl Default for AxonId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for Axon {
    fn default() -> Self {
        Self {
            id: AxonId::default(),
            inner: Arc::new(RwLock::new(AxonInner::default())),
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
