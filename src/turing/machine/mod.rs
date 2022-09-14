//! All parts assembly into a single machine
use super::{
    program::{Action, Key, Lookup, Program},
    state::State,
    symbol::Symbol,
    tape::Tape,
};

/// A Turing machine
pub struct Machine<'a, T>
where
    T: Tape + Sized,
{
    tape: T,
    program: &'a dyn Program,
    state: State,
}

impl<'a, T> From<Machine<'a, T>> for (T, State, &'a dyn Program)
where
    T: Tape + Sized,
{
    fn from(machine: Machine<'a, T>) -> Self {
        (machine.tape, machine.state, machine.program)
    }
}

impl<'a, T> Machine<'a, T>
where
    T: Tape + Sized,
{
    /// Create a Turing machine with a certain tape and program. Starts in state 0
    pub fn new(tape: T, program: &'a dyn Program) -> Self {
        Self {
            tape,
            state: State::Number(0),
            program,
        }
    }

    /// Create a Turing machine with a certain tape, state and program.
    pub fn with(tape: T, state: State, program: &'a dyn Program) -> Self {
        Self {
            tape,
            state,
            program,
        }
    }

    /// Take a single step
    pub fn step(&mut self) -> Progress {
        if !self.state.halted() {
            let key = Key {
                state: self.state,
                symbol: self.tape.read(),
            };
            match self.program.lookup(&key) {
                Lookup::Unknown => Progress::Stuck,
                Lookup::Indeterminate => Progress::Limbo,
                Lookup::Determined(Action::Halt) => {
                    self.state = State::Halted;
                    Progress::Made
                }
                Lookup::Determined(Action::Do {
                    symbol,
                    direction,
                    state,
                }) => {
                    self.tape.write(symbol);
                    self.tape.move_to(&direction);
                    self.state = state;
                    Progress::Made
                }
            }
        } else {
            Progress::Halted
        }
    }

    /// Take several steps until either the maximum number of steps is attained or the machine halted.
    pub fn run(&mut self, maximum_steps: u128) -> Assessment {
        let mut steps_taken: u128 = 0u128;
        while !self.state.halted() && steps_taken < maximum_steps {
            let progress = self.step();
            if matches!(progress, Progress::Made) {
                steps_taken += 1;
            } else {
                return Assessment::NoProgress(
                    progress,
                    Details {
                        steps: steps_taken,
                        score: 0,
                        multiplicity: self.program.multiplicity(),
                    },
                );
            }
        }
        if self.state.halted() {
            Assessment::HaltedIn(Details {
                steps: steps_taken,
                score: self.score(),
                multiplicity: self.program.multiplicity(),
            })
        } else {
            Assessment::NotHalted(Details {
                steps: steps_taken,
                score: 0,
                multiplicity: self.program.multiplicity(),
            })
        }
    }

    /// The number of non blank symbols on the tape
    pub fn score(&self) -> usize {
        self.tape.count(&Symbol::NonBlank)
    }
}

/// The possibilities when a Turing machine takes a step
#[derive(Debug, PartialEq, Eq)]
pub enum Progress {
    /// The Turing machine could have halted.
    Halted,
    /// The Turing machine could have gotten stuck. TODO describe when this happens
    Stuck,
    /// The Turing machine could not progress, because of an incomplete program
    Limbo,
    /// The Turing machine could have made progress
    Made,
}

/// The possibilities when a Turing machine is run.
#[derive(Debug, PartialEq, Eq)]
pub enum Assessment {
    /// The Turing machine could not make progress after a certain number of steps
    NoProgress(Progress, Details),
    /// The Turing machine could have halted
    HaltedIn(Details),
    /// The Turing machine is still running after a maximum number of steps is reached
    NotHalted(Details),
}

/// Details of a run
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Details {
    /// Number of steps taken
    pub steps: u128,
    /// Number of non blank symbols on the tape
    pub score: usize,
    /// The multiplicity of the program that was used to run the Turing machine
    pub multiplicity: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing::{direction::Direction, program::CompleteProgram, tape::SimpleTape};

    #[test]
    fn a_simple_machine_can_be_run() {
        let mut program = CompleteProgram::new();
        program.insert(
            (State::Number(0), Symbol::Blank),
            (Symbol::NonBlank, Direction::Right, State::Number(1)),
        );
        program.insert(
            (State::Number(0), Symbol::NonBlank),
            (Symbol::NonBlank, Direction::Left, State::Halted),
        );
        program.insert(
            (State::Number(1), Symbol::Blank),
            (Symbol::NonBlank, Direction::Left, State::Number(0)),
        );
        program.insert(
            (State::Number(1), Symbol::NonBlank),
            (Symbol::NonBlank, Direction::Right, State::Halted),
        );
        let start = State::Number(0);
        let mut machine = Machine::with(SimpleTape::empty(), start, &program);

        let steps = machine.run(10);

        assert_eq!(
            steps,
            Assessment::HaltedIn(Details {
                steps: 3,
                score: 2,
                multiplicity: 1
            })
        );
    }
}
