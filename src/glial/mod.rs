use crate::neuron::Neuron;

#[doc = r#"

## Notes
Glial cells are the caretakers of the brain.

They tells Neurons where to grow to and retract from.


## Ideas

Maybe the glial cells need to monitor if the neurons are firing together, etc.

"#]
pub struct Glial {
    monitoring: Vec<Neuron>,
}

impl Glial {
    pub fn monitor(&mut self, neuron: Neuron) -> &mut Self {
        self.monitoring.push(neuron);
        self
    }
}

impl Default for Glial {
    fn default() -> Self {
        Self {
            monitoring: Vec::new(),
        }
    }
}

#[test]
pub fn simple_glial() {
    let mut glial = Glial::default();

    let n1 = Neuron::default();
    let n2 = Neuron::default();

    glial.monitor(n1).monitor(n2);
}
