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
}
