use crate::turing::direction;
use crate::turing::direction::{Direction, Directions};
use crate::turing::state;
use crate::turing::state::{State, States};
use crate::turing::symbol;
use crate::turing::symbol::{Symbol, Symbols};
use cartesian::*;
use std::convert::{From, Into};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::once;
use std::str::FromStr;

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

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.len() == 3 {
            match input {
                "  H" => Ok(Action::Halt),
                _ => {
                    let symbol = input[0..1]
                        .parse::<Symbol>()
                        .map_err(ParseError::SymbolProblem)?;
                    let direction = input[1..2]
                        .parse::<Direction>()
                        .map_err(ParseError::DirectionProblem)?;
                    let state = input[2..3]
                        .parse::<State>()
                        .map_err(ParseError::StateProblem)?;

                    Ok(Action::Do {
                        symbol,
                        direction,
                        state,
                    })
                }
            }
        } else {
            Err(ParseError::IncorrectLength(input.len()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    IncorrectLength(usize),
    SymbolProblem(symbol::ParseError),
    DirectionProblem(direction::ParseError),
    StateProblem(state::ParseError),
    UnknownSymbol(String),
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
    fn actions_can_be_parsed() {
        assert_eq!(Ok(Action::Halt), "  H".parse());
        assert_eq!(
            Ok(Action::Do {
                symbol: Symbol::Blank,
                direction: Direction::Left,
                state: State::Number(2)
            }),
            "0L2".parse()
        );
    }

    #[test]
    fn parse_checks_for_errors() {
        assert_eq!(Err(ParseError::IncorrectLength(2)), "..".parse::<Action>());
        assert_eq!(
            Err(ParseError::IncorrectLength(4)),
            "....".parse::<Action>()
        );
        assert_eq!(
            Err(ParseError::SymbolProblem(
                symbol::ParseError::UnknownSymbol(" ".to_owned())
            )),
            " L1".parse::<Action>()
        );
        assert_eq!(
            Err(ParseError::DirectionProblem(
                direction::ParseError::UnknownSymbol("H".to_owned())
            )),
            "0H1".parse::<Action>()
        );
        assert_eq!(
            Err(ParseError::StateProblem(state::ParseError::UnknownState(
                "a".to_owned()
            ))),
            "0La".parse::<Action>()
        );
    }

    #[test]
    fn actions_up_to_contain_all_actions_up_to_maximum() {
        let actual: Vec<Action> = Actions::up_to(2).collect();

        assert_eq!(
            vec![
                Action::Halt,
                (Symbol::Blank, Direction::Left, State::Number(0)).into(),
                (Symbol::Blank, Direction::Right, State::Number(0)).into(),
                (Symbol::NonBlank, Direction::Left, State::Number(0)).into(),
                (Symbol::NonBlank, Direction::Right, State::Number(0)).into(),
                (Symbol::Blank, Direction::Left, State::Number(1)).into(),
                (Symbol::Blank, Direction::Right, State::Number(1)).into(),
                (Symbol::NonBlank, Direction::Left, State::Number(1)).into(),
                (Symbol::NonBlank, Direction::Right, State::Number(1)).into(),
            ],
            actual
        )
    }
}
