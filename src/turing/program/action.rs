use crate::turing::direction::{Direction, Directions};
use crate::turing::state::{State, States};
use crate::turing::symbol::{Symbol, Symbols};
use cartesian::*;
use std::convert::{From, Into};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::once;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    Halt,
    Do {
        symbol: Symbol,
        direction: Direction,
        state: State,
    },
}

impl From<(Symbol, Direction, State)> for Action {
    fn from(action: (Symbol, Direction, State)) -> Self {
        Action::Do {
            symbol: action.0,
            direction: action.1,
            state: action.2,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Action::Halt => write!(f, "  H"),
            Action::Do {
                symbol,
                direction,
                state,
            } => write!(f, "{}{}{}", symbol, direction, state),
        }
    }
}

pub struct Actions {
    iterator: Box<dyn Iterator<Item = Action>>,
}

impl Actions {
    pub fn up_to(maximum: u8) -> Self {
        let iterator =
            once(Action::Halt).chain(States::non_halted_up_to(maximum).flat_map(|state| {
                cartesian!(Symbols::all(), Directions::all()).map(move |tuple| {
                    let action: Action = (tuple.0, tuple.1, state).into();
                    action
                })
            }));
        Self {
            iterator: Box::new(iterator),
        }
    }
}

impl Iterator for Actions {
    type Item = Action;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_up_to_contain_all_actions_up_to_maximum() {
        let actual: Vec<Action> = Actions::up_to(1).collect();

        assert_eq!(
            vec![
                Action::Halt,
                (Symbol::Blank, Direction::Left, State::Number(0)).into(),
                (Symbol::Blank, Direction::Right, State::Number(0)).into(),
                (Symbol::NonBlank, Direction::Left, State::Number(0)).into(),
                (Symbol::NonBlank, Direction::Right, State::Number(0)).into(),
            ],
            actual
        )
    }
}
