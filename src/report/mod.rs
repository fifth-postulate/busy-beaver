use crate::turing::{Assessment, Details};

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    subjects: usize,
    total: usize,
    halted: usize,
    indeterminate: usize,
    stuck: usize,
    sigma_champion: Option<Champion>,
    s_champion: Option<Champion>,
}

impl Report {
    pub fn new() -> Self {
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

    pub fn update_with(&mut self, assessment: &Assessment) {
        match assessment {
            Assessment::HaltedIn(details) => {
                self.halted(*details);
            }
            Assessment::NoProgress(_reason, details) => {
                self.stuck(*details);
            }
            Assessment::NotHalted(details) => {
                self.indeterminated(*details);
            }
        };
    }

    pub fn halted(&mut self, details: Details) {
        self.subjects += 1;
        self.total += details.multiplicity;
        self.halted += details.multiplicity;
        self.update_champion(details);
    }

    pub fn indeterminated(&mut self, details: Details) {
        self.subjects += 1;
        self.total += details.multiplicity;
        self.indeterminate += details.multiplicity;
    }

    pub fn stuck(&mut self, details: Details) {
        self.subjects += 1;
        self.total += details.multiplicity;
        self.stuck += details.multiplicity;
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

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Champion {
    details: Details,
    count: usize,
}

impl Champion {
    pub fn new(details: Details) -> Self {
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
