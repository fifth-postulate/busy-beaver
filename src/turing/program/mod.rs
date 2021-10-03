mod action;
mod key;

pub use action::{Action, Actions};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo() {
        assert_eq!(1 + 1, 2)
    }
}
