use std::sync::{Arc, Mutex};

use uuid::Uuid;

pub struct Dendrite {
    id: Uuid,
    inner: Arc<Mutex<DendriteInner>>,
}

struct DendriteInner;
