//! Gather statistics about a collection of Turing machines.
//!
//! When analyzing a collection of Turing machines in the context of the busy beaver problem we are interested in the following statistics
//! * How many subjects are analyzed.
//! * How many programs are represented by the subjects.
//! * How many subjects halted.
//! * How many subjects didn't halt within the alloted running time.
//! * How many subjects got stuck.
//! * What is a current sigma champion
//! * What is a current s champion
//!
//! The following code is representative for how to gather statistics
//!
//! ```
//! # use busy_beaver::{turing::{SimpleTape, Programs, Machine}, report::Report};
//! let mut report = Report::new();
//! for program in Programs::all(2) {
//!     let mut machine = Machine::new(SimpleTape::empty(), &program);
//!     let assessment = machine.run(100);
//!     report.update_with(&assessment);
//! }
//! ```
use crate::turing::{Assessment, Details};

/// Summary of information about the Turing machines under consideration.
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
    /// Create an empty report
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

    /// Update the report with the assessment of the progress of a Turing machine.
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

    /// Update the report with the details of a **halted** Turing machine.
    pub fn halted(&mut self, details: Details) {
        self.subjects += 1;
        self.total += details.multiplicity;
        self.halted += details.multiplicity;
        self.update_champion(details);
    }

    /// Update the report with the details of a Turing machine whose behaviour is **indeterminate**.
    pub fn indeterminated(&mut self, details: Details) {
        self.subjects += 1;
        self.total += details.multiplicity;
        self.indeterminate += details.multiplicity;
    }

    /// Update the report with the details of a Turing machine that did not progess.
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

/// A *champion* is a Turing machine that out performs other Turing machines in a certain category.
#[derive(Debug, PartialEq, Eq)]
pub struct Champion {
    details: Details,
    count: usize,
}

impl Champion {
    /// Creates a new champion from its corresponding details
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
