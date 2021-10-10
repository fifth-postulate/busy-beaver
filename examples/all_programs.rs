use busy_beaver::turing::Programs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u8 = args
        .get(1)
        .and_then(|input| input.parse().ok())
        .unwrap_or(2);

    let mut count = 0;
    for program in Programs::all(n) {
        println!("{}", program);
        count += 1;
    }
    println!("{}", count);
}
