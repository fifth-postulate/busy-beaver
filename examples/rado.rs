use busy_beaver::turing::{Direction, Machine, Program, State, Symbol};
use std::time::Instant;

fn main() {
    let mut program = Program::new();
    program.insert(
        (State::Number(0), Symbol::Blank),
        (Symbol::NonBlank, Direction::Left, State::Number(1)),
    );
    program.insert(
        (State::Number(0), Symbol::NonBlank),
        (Symbol::NonBlank, Direction::Right, State::Number(2)),
    );
    program.insert(
        (State::Number(1), Symbol::Blank),
        (Symbol::NonBlank, Direction::Right, State::Number(0)),
    );
    program.insert(
        (State::Number(1), Symbol::NonBlank),
        (Symbol::NonBlank, Direction::Left, State::Number(1)),
    );
    program.insert(
        (State::Number(2), Symbol::Blank),
        (Symbol::NonBlank, Direction::Right, State::Number(1)),
    );
    program.insert(
        (State::Number(2), Symbol::NonBlank),
        (Symbol::NonBlank, Direction::Left, State::Halted),
    );
    let mut machine = Machine::new(State::Number(0), program);

    let start = Instant::now();
    let steps = machine.run(10_000);
    let duration = start.elapsed();

    println!("{:?} steps taken in {:?}", steps, duration);
}
