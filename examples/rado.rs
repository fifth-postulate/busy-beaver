use busy_beaver::turing::{Machine, Program};
use std::time::Instant;

fn main() {
    let program: Program = "1L1 1R2 1R0 1L1 1R1 1LH".parse().unwrap();
    let mut machine = Machine::new(program);

    let start = Instant::now();
    let steps = machine.run(100);
    let duration = start.elapsed();

    println!("{:?} in {:?}", steps, duration);
}
