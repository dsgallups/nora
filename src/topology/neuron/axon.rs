use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::prelude::Dendrite;

#[doc = r#"


## Notes
one thing about axons IRL is that they can have different action potentials and durations.

Assumption is that this is a result of the medium (electricity) and
doesn't necessarily have intrinsic value that can't be expressed otherwise.
"#]
pub struct Axon {
    id: AxonId,
    connections: Vec<Dendrite>,
}

impl Axon {
    pub fn attach_to(&mut self, dendritic_receptor: &Dendrite) {
        self.connections.push(dendritic_receptor.clone())
    }
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
