use busy_beaver::turing::{Machine, Program, State};
use std::time::Instant;

fn main() {
    let program: Program = "1L1 1R2 1R0 1L1 1R1 1LH".parse().unwrap();
    let mut machine = Machine::new(State::Number(0), program);

    let start = Instant::now();
    let steps = machine.run(100);
    let duration = start.elapsed();

    println!("{:?} in {:?}", steps, duration);
}
