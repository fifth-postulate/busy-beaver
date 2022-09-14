#![deny(missing_docs)]
//! This library allows one to explore the [busy beaver problem](https://en.wikipedia.org/wiki/Busy_beaver).

pub mod graph;
pub mod report;
pub mod turing;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
