#![deny(missing_docs)]
//! This library allows one to explore the [busy beaver problem](https://en.wikipedia.org/wiki/Busy_beaver).
//!
//! The busy beaver problem is to determine the maximum running time of a halting [Turing machine](https://en.wikipedia.org/wiki/Turing_machine)
//! with a certain number of states. Tibor Rado introduced the concept in 1962 and showed that the busy beaver function is uncomputable, i.e.
//! there is no Turing machine that computes the maximum running time of an n-state Turing machine.

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
