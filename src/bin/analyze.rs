use busy_beaver::turing::{Assessment, Details, Machine, Programs};
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

#[derive(Debug, PartialEq, Eq)]
struct Report {
    total: usize,
    halted: usize,
    indeterminate: usize,
    sigma_champion: Option<Champion>,
    s_champion: Option<Champion>,
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

impl Report {
    fn new() -> Self {
        Self {
            total: 0,
            halted: 0,
            indeterminate: 0,
            s_champion: None,
            sigma_champion: None,
        }
    }

    fn update_with(&mut self, assessment: &Assessment) {
        self.total += 1;
        match assessment {
            Assessment::HaltedIn(details) => {
                self.halted += 1;
                match &mut self.sigma_champion {
                    Some(reigning) => {
                        if details.score > reigning.details.score {
                            reigning.update(*details);
                        }
                        if details.score == reigning.details.score {
                            reigning.tally();
                        }
                    }
                    None => {
                        self.sigma_champion = Some(Champion::new(*details));
                    }
                };
                match &mut self.s_champion {
                    Some(reigning) => {
                        if details.steps > reigning.details.steps {
                            reigning.update(*details);
                        }
                        if details.steps == reigning.details.steps {
                            reigning.tally();
                        }
                    }
                    None => {
                        self.s_champion = Some(Champion::new(*details));
                    }
                };
            }
            Assessment::NotHalted => {
                self.indeterminate += 1;
            }
        };
    }
}
