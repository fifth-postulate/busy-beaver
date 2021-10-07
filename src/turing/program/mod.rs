mod action;
mod key;

pub use action::{Action, Actions};
use cartesian::*;
pub use key::{Key, Keys};
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Program {
    program: Vec<Action>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            program: Vec::new(),
        }
    }

    pub fn insert<K, A>(&mut self, key: K, action: A)
    where
        K: Into<Key>,
        A: Into<Action>,
    {
        let key = key.into();
        self.program.insert(key.idx(), action.into());
    }

    pub fn get(&self, key: &Key) -> Option<&Action> {
        self.program.get(key.idx())
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Program {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let n = (self.program.len() / 2) as u8; // We are assume only complete programs
        let actions: Vec<String> = Keys::up_to(n)
            .map(|k| self.get(&k))
            .map(|ao| ao.map(|a| a.to_string()).unwrap_or("???".to_string()))
            .collect();
        formatter.write_str(&actions.join(" "))
    }
}

pub struct Programs {
    iterator: Box<dyn Iterator<Item = Program>>,
}

impl Programs {
    pub fn all1() -> Self {
        let iterator = cartesian!(Actions::up_to(1), Actions::up_to(1))
            .map(|tuple| vec![tuple.0, tuple.1])
            .map(|actions| {
                let mut program = Program::new();
                for (key, action) in Keys::up_to(1).zip(actions) {
                    program.insert(key, action);
                }
                program
            });
        Self {
            iterator: Box::new(iterator),
        }
    }

    pub fn all2() -> Self {
        let iterator = cartesian!(
            Actions::up_to(2),
            Actions::up_to(2),
            Actions::up_to(2),
            Actions::up_to(2)
        )
        .map(|tuple| vec![tuple.0, tuple.1, tuple.2, tuple.3])
        .map(|actions| {
            let mut program = Program::new();
            for (key, action) in Keys::up_to(2).zip(actions) {
                program.insert(key, action);
            }
            program
        });
        Self {
            iterator: Box::new(iterator),
        }
    }

    pub fn all3() -> Self {
        let iterator = cartesian!(
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3),
            Actions::up_to(3)
        )
        .map(|tuple| vec![tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5])
        .map(|actions| {
            let mut program = Program::new();
            for (key, action) in Keys::up_to(3).zip(actions) {
                program.insert(key, action);
            }
            program
        });
        Self {
            iterator: Box::new(iterator),
        }
    }

    pub fn all4() -> Self {
        let iterator = cartesian!(
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4),
            Actions::up_to(4)
        )
        .map(|tuple| {
            vec![
                tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7,
            ]
        })
        .map(|actions| {
            let mut program = Program::new();
            for (key, action) in Keys::up_to(4).zip(actions) {
                program.insert(key, action);
            }
            program
        });
        Self {
            iterator: Box::new(iterator),
        }
    }

    pub fn all5() -> Self {
        let iterator = cartesian!(
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5),
            Actions::up_to(5)
        )
        .map(|tuple| {
            vec![
                tuple.0, tuple.1, tuple.2, tuple.3, tuple.4, tuple.5, tuple.6, tuple.7, tuple.8,
                tuple.9,
            ]
        })
        .map(|actions| {
            let mut program = Program::new();
            for (key, action) in Keys::up_to(5).zip(actions) {
                program.insert(key, action);
            }
            program
        });
        Self {
            iterator: Box::new(iterator),
        }
    }
}

impl Iterator for Programs {
    type Item = Program;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all1_contains_correct_number_of_programs() {
        let number_of_programs = Programs::all1().count();

        assert_eq!(number_of_programs, 25);
    }

    #[test]
    fn all2_contains_correct_number_of_programs() {
        let number_of_programs = Programs::all2().count();

        assert_eq!(number_of_programs, 6561);
    }
}
