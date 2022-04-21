use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Symbol {
    Blank,
    NonBlank,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Blank
    }
}

impl Default for &Symbol {
    fn default() -> Self {
        &Symbol::Blank
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Symbol::Blank => write!(f, "0"),
            Symbol::NonBlank => write!(f, "1"),
        }
    }
}

impl From<usize> for Symbol {
    fn from(index: usize) -> Self {
        match index % 2 {
            1 => Symbol::NonBlank,
            _ => Symbol::Blank,
        }
    }
}

impl FromStr for Symbol {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "0" => Ok(Symbol::Blank),
            "1" => Ok(Symbol::NonBlank),
            _ => Err(ParseError::UnknownSymbol(input.to_owned())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownSymbol(String),
}

pub struct Symbols {
    current: Option<Symbol>,
}

impl Symbols {
    pub fn all() -> Self {
        Self {
            current: Some(Symbol::Blank),
        }
    }
}

impl Iterator for Symbols {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.current;
        self.current = match item {
            Some(Symbol::Blank) => Some(Symbol::NonBlank),
            _ => None,
        };
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_symbols_are_equal() {
        assert_eq!(Symbol::Blank, Symbol::Blank);
        assert_eq!(Symbol::NonBlank, Symbol::NonBlank);
    }

    #[test]
    fn distinct_symbols_are_non_equal() {
        assert_ne!(Symbol::Blank, Symbol::NonBlank);
        assert_ne!(Symbol::NonBlank, Symbol::Blank);
    }

    #[test]
    fn symbols_can_be_parsed() {
        assert_eq!(Ok(Symbol::Blank), "0".parse());
        assert_eq!(Ok(Symbol::NonBlank), "1".parse());
    }

    #[test]
    fn symbols_all_contain_all_symbols() {
        let actual: Vec<Symbol> = Symbols::all().collect();

        assert_eq!(vec![Symbol::Blank, Symbol::NonBlank], actual);
    }
}
