mod dendrite;
pub use dendrite::*;

mod junction;
pub use junction::*;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct NeuronInput(Vec<Afference>);

impl NeuronInput {
    pub fn push(&mut self, afference: impl Into<Afference>) {
        self.0.push(afference.into())
    }
    pub fn swap_remove(&mut self, index: usize) -> Afference {
        self.0.swap_remove(index)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Afference> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Afference> {
        self.0.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn prune_the_dead(&mut self) {
        self.0.retain(|dendrite| !dendrite.is_dead())
    }
    pub fn find(&self, id: Uuid) -> Option<&Afference> {
        self.0.iter().find(|d| d.id() == id)
    }
}

#[derive(Debug)]
pub enum Afference {
    Dendrite(Dendrite),
    Junction(JunctionAffer),
}
impl Afference {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Dendrite(d) => d.id(),
            Self::Junction(j) => j.id(),
        }
    }
    pub fn is_dead(&self) -> bool {
        match self {
            Self::Dendrite(d) => d.is_dead(),
            Self::Junction(_) => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Disconnected;
