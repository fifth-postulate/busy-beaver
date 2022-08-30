use busy_beaver::turing::{CompleteProgram, Machine, SimpleTape};
use std::time::Instant;

fn main() {
    let program: CompleteProgram = "1L1 1R2 1R0 1L1 1R1 1LH".parse().unwrap();
    let mut machine = Machine::new(SimpleTape::empty(), &program);

    let start = Instant::now();
    let steps = machine.run(100);
    let duration = start.elapsed();

    println!("{:?} in {:?}", steps, duration);
}
