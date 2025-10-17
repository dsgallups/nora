use std::sync::{Arc, RwLock};

use uuid::Uuid;

pub struct Dendrite {
    id: DendriteId,
    inner: Arc<RwLock<DendriteInner>>,
}

impl Clone for Dendrite {
    fn clone(&self) -> Self {
        Dendrite {
            id: self.id,
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Default for Dendrite {
    fn default() -> Self {
        Self {
            id: DendriteId::default(),
            inner: Arc::new(RwLock::new(DendriteInner::default())),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct DendriteId(Uuid);

impl Default for DendriteId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

struct DendriteInner {
    /// If its attached axon has fired
    incoming_action_potential: bool,
}

impl Default for DendriteInner {
    fn default() -> Self {
        Self {
            incoming_action_potential: false,
        }
    }
}
