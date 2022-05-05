use busy_beaver::{
    report::Report,
    turing::{Machine, Programs},
};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u8 = args
        .get(1)
        .and_then(|input| input.parse().ok())
        .unwrap_or(2);
    let maximum: u128 = args
        .get(2)
        .and_then(|input| input.parse().ok())
        .unwrap_or(10_000);

    let mut report = Report::new();
    let start = Instant::now();
    for program in Programs::all(n) {
        print!(".");
        let mut machine = Machine::new(&program);
        let assessment = machine.run(maximum);
        report.update_with(&assessment);
    }
    let duration = start.elapsed();

    println!(
        "\nn={}, maximum={}, duration={:?}: {:?}",
        n, maximum, duration, report
    );
}
