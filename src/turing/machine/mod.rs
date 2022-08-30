use super::{
    program::{Action, Key, Lookup, Program},
    state::State,
    symbol::Symbol,
    tape::Tape,
};

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
    pub fn new(tape: T, program: &'a dyn Program) -> Self {
        Self {
            tape,
            state: State::Number(0),
            program,
        }
    }

    pub fn with(tape: T, state: State, program: &'a dyn Program) -> Self {
        Self {
            tape,
            state,
            program,
        }
    }

    pub fn step(&mut self) -> Progress {
        if !self.state.halted() {
            let key = Key {
                state: self.state,
                symbol: *self.tape.read(),
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

    pub fn score(&self) -> usize {
        self.tape.count(&Symbol::NonBlank)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Progress {
    Halted,
    Stuck,
    Limbo,
    Made,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Assessment {
    NoProgress(Progress, Details),
    HaltedIn(Details),
    NotHalted(Details),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Details {
    pub steps: u128,
    pub score: usize,
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
