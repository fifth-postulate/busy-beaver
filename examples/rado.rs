use busy_beaver::turing::{Machine, CompleteProgram};
use std::time::Instant;

fn main() {
    let program: CompleteProgram = "1L1 1R2 1R0 1L1 1R1 1LH".parse().unwrap();
    let mut machine = Machine::new(&program);

    let start = Instant::now();
    let steps = machine.run(100);
    let duration = start.elapsed();

    println!("{:?} in {:?}", steps, duration);
}
