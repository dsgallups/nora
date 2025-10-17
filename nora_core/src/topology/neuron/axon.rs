use std::fmt;

use trotcast::{Channel, error::SendError};

use crate::prelude::{NeuronChannel, NeuronRx};

#[doc = r#"


## Notes
one thing about axons IRL is that they can have different action potentials and durations.

Assumption is that this is a result of the medium (electricity) and
doesn't necessarily have intrinsic value that can't be expressed otherwise.
"#]
pub struct Axon {
    name: String,
    channel: NeuronChannel,
}
impl fmt::Debug for Axon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Axon").field("name", &self.name).finish()
    }
}

impl Axon {
    pub fn new(name: impl Into<String>) -> Self {
        let channel = Channel::new(20);
        Self {
            name: name.into(),
            channel,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn spawn_rx(&self) -> NeuronRx {
        self.channel.spawn_rx()
    }
    pub fn fire(&self, value: u8) -> Result<(), SendError<u8>> {
        self.channel.send(value)
    }
}
