use busy_beaver::turing::{Actions, Keys, Program};
use cartesian::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u8 = args
        .get(1)
        .and_then(|input| input.parse().ok())
        .unwrap_or(2);

    let mut count = 0;
    for program in cartesian!(
        Actions::up_to(n),
        Actions::up_to(n),
        Actions::up_to(n),
        Actions::up_to(n)
    )
    .map(|tuple| vec![tuple.0, tuple.1, tuple.2, tuple.3])
    .map(|actions| {
        let mut program = Program::new();
        for (key, action) in Keys::up_to(2).zip(actions) {
            program.insert(key, action);
        }
        program
    }) {
        println!("{}", program);
        count += 1;
    }
    println!("{}", count);
}
