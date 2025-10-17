use nora::prelude::*;

fn main() {
    let mut brain = Brain::sandbox();

    let input = NumJunction::new(2);
    let output = NumJunction::new(0);

    brain.affer(&input);
    brain.effer(&output);

    for _ in 0..10 {
        brain.step();
        println!("output: {:?}", output.val())
    }
}
