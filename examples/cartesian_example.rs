use busy_beaver::turing::{Action, Directions, States, Symbols};
use cartesian::*;
use std::convert::From;
use std::env;
use std::iter::once;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u8 = args
        .get(1)
        .and_then(|input| input.parse().ok())
        .unwrap_or(5);

    let mut count = 0;
    let iterator = once(Action::Halt).chain(States::non_halted_up_to(n).flat_map(|state| {
        cartesian!(Symbols::all(), Directions::all()).map(move |tuple| {
            let action: Action = From::from((tuple.0, tuple.1, state));
            action
        })
    }));
    for state in iterator {
        println!("{:?}", state);
        count += 1;
    }
    println!("{}", count);
}
