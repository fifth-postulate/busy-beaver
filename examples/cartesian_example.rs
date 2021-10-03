use busy_beaver::turing::{Action, Directions, States, Symbols};
use cartesian::*;
use std::convert::{From, Into};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u8 = args.get(1).and_then(|input| input.parse().ok()).unwrap_or(5);

    let mut count = 0;
    for state in States::up_to(n) {
        if state.halted() {
            println!("{:?}", Action::Halt);
            count += 1;
        } else {
            for tuple in cartesian!(Symbols::all(), Directions::all()) {
                let action: Action = From::from((tuple.0, tuple.1, state));
                println!("{:?}", action);
                count += 1;
            }
        }
    }
    println!("{}", count);
}
