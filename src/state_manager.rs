use crate::brain::Brain;

pub struct StateManager {
    brain: Brain,
}

impl StateManager {
    pub fn sandbox(brain: Brain) -> Self {
        Self { brain }
    }
}
