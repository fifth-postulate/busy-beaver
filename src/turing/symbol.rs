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

struct Symbols {
    current: Option<Symbol>,
}

impl Symbols {
    fn all() -> Self {
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
    fn distinct_symbols_are_distinct() {
        assert_ne!(Symbol::Blank, Symbol::NonBlank);
        assert_ne!(Symbol::NonBlank, Symbol::Blank);
    }

    #[test]
    fn symbols_all_contain_all_symbols() {
        let actual: Vec<Symbol> = Symbols::all().collect();

        assert_eq!(vec![Symbol::Blank, Symbol::NonBlank], actual);
    }
}
