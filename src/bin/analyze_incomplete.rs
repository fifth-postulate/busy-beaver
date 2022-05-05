use busy_beaver::turing::{
    Assessment, Details, IncompleteProgram, Key, Machine, Program, Progress, State, Tape,
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
    let mut candidates: Vec<(u128, Tape, State, IncompleteProgram)> = vec![(
        0,
        Tape::empty(),
        State::Number(0),
        IncompleteProgram::with_states(n),
    )];
    let start = Instant::now();
    while let Some((steps_taken, tape, state, program)) = candidates.pop() {
        println!("{}", program);
        let mut step_count = steps_taken;
        let mut machine: Machine = Machine::with(tape, state, &program);
        loop {
            let progress = machine.step();

            match progress {
                Progress::Made => {
                    //print!("·");
                    step_count += 1;
                    if step_count >= maximum {
                        report.indeterminated(program.multiplicity());
                        break;
                    }
                }
                Progress::Halted => {
                    //print!("⊞");
                    let details = Details {
                        steps: step_count,
                        score: machine.score(),
                        multiplicity: program.multiplicity(),
                    };
                    report.halted(details);
                    break;
                }
                Progress::Limbo => {
                    let (t, s, _): (Tape, State, &dyn Program) = machine.into();
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

#[derive(Debug, PartialEq, Eq)]
struct Report {
    subjects: usize,
    total: usize,
    halted: usize,
    indeterminate: usize,
    stuck: usize,
    sigma_champion: Option<Champion>,
    s_champion: Option<Champion>,
}

impl Report {
    fn new() -> Self {
        Self {
            subjects: 0,
            total: 0,
            halted: 0,
            indeterminate: 0,
            stuck: 0,
            s_champion: None,
            sigma_champion: None,
        }
    }

    fn halted(&mut self, details: Details) {
        self.subjects += 1;
        self.total += details.multiplicity;
        self.halted += details.multiplicity;
        self.update_champion(details);
    }

    fn indeterminated(&mut self, multiplicity: usize) {
        self.subjects += 1;
        self.total += multiplicity;
        self.indeterminate += multiplicity;
    }

    fn update_champion(&mut self, details: Details) {
        match &mut self.sigma_champion {
            Some(reigning) => {
                if details.score > reigning.details.score {
                    reigning.update(details);
                }
                if details.score == reigning.details.score {
                    reigning.tally();
                }
            }
            None => {
                self.sigma_champion = Some(Champion::new(details));
            }
        };
        match &mut self.s_champion {
            Some(reigning) => {
                if details.steps > reigning.details.steps {
                    reigning.update(details);
                }
                if details.steps == reigning.details.steps {
                    reigning.tally();
                }
            }
            None => {
                self.s_champion = Some(Champion::new(details));
            }
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Champion {
    details: Details,
    count: usize,
}

impl Champion {
    fn new(details: Details) -> Self {
        Self { details, count: 1 }
    }

    fn update(&mut self, details: Details) {
        self.details = details;
        self.count = 1;
    }

    fn tally(&mut self) {
        self.count += 1;
    }
}
