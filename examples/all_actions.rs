use busy_beaver::turing::Actions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u8 = args
        .get(1)
        .and_then(|input| input.parse().ok())
        .unwrap_or(5);

    let mut count = 0;
    for state in Actions::up_to(n) {
        println!("{:?}", state);
        count += 1;
    }
    println!("{}", count);
}
