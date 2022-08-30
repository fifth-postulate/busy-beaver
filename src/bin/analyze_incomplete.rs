use busy_beaver::{
    report::Report,
    turing::{Details, IncompleteProgram, Machine, Program, Progress, SimpleTape, State, Tape},
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
    let mut candidates: Vec<(u128, SimpleTape, State, IncompleteProgram)> = vec![(
        0,
        SimpleTape::empty(),
        State::Number(0),
        IncompleteProgram::with_states(n),
    )];
    let start = Instant::now();
    while let Some((steps_taken, tape, state, program)) = candidates.pop() {
        print!(".");
        let mut step_count = steps_taken;
        let mut machine = Machine::with(tape, state, &program);
        loop {
            let progress = machine.step();

            match progress {
                Progress::Made => {
                    step_count += 1;
                    if step_count >= maximum {
                        report.indeterminated(Details {
                            steps: step_count,
                            score: 0,
                            multiplicity: program.multiplicity(),
                        });
                        break;
                    }
                }
                Progress::Halted => {
                    let details = Details {
                        steps: step_count,
                        score: machine.score(),
                        multiplicity: program.multiplicity(),
                    };
                    report.halted(details);
                    break;
                }
                Progress::Limbo => {
                    let (t, s, _): (SimpleTape, State, &dyn Program) = machine.into();
                    for p in program.extentions((s, *t.read())) {
                        candidates.push((step_count, t.clone(), s, p));
                    }
                    break;
                }
                Progress::Stuck => {
                    panic!("Incomplete programs should not get stuck.")
                }
            }
        }
    }
    let duration = start.elapsed();

    println!(
        "\nn={}, maximum={}, duration={:?}: {:?}",
        n, maximum, duration, report
    );
}
