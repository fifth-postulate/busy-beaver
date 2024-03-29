use busy_beaver::turing::{CompleteProgram, Machine, SimpleTape as Tape};
use std::time::Instant;

fn main() {
    let program: CompleteProgram = "1R1 1L2 1R2 1R1 1R3 0L4 1L0 1L3 1RH 0L0".parse().unwrap();
    let mut machine = Machine::new(Tape::empty(), &program);

    let start = Instant::now();
    let assessment = machine.run(50_000_000);
    let duration = start.elapsed();

    println!("{:?} in {:?}", assessment, duration);
}
