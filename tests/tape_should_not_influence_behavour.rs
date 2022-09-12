use busy_beaver::turing::{CompleteProgram, CompoundTape, Machine, Programs, SimpleTape};

fn compound_tape_results_in_same_behavour_as_simple_tape_on_various_programs() {
    compound_tape_should_result_in_same_behavour_as_simple_tape_on("0L1   H 1R0 1L1", 10);
}

fn compound_tape_should_result_in_same_behavour_as_simple_tape_on(
    description: &str,
    maximum_steps: u128,
) {
    let program = description.parse::<CompleteProgram>().unwrap();
    let mut machine_with_simple_tape = Machine::new(SimpleTape::empty(), &program);
    let mut machine_with_compound_tape = Machine::new(CompoundTape::empty(), &program);

    let assessment_of_simple_tape = machine_with_simple_tape.run(maximum_steps);
    let assessment_of_compound_tape = machine_with_compound_tape.run(maximum_steps);

    assert_eq!(assessment_of_simple_tape, assessment_of_compound_tape);
}

fn compound_tape_results_in_same_behavour_as_simple_tape_for_all_progams_with_two_states() {
    for program in Programs::all(2) {
        println!("{}", program);
        let mut machine_with_simple_tape = Machine::new(SimpleTape::empty(), &program);
        let mut machine_with_compound_tape = Machine::new(CompoundTape::empty(), &program);

        let assessment_of_simple_tape = machine_with_simple_tape.run(100);
        let assessment_of_compound_tape = machine_with_compound_tape.run(100);

        assert_eq!(assessment_of_simple_tape, assessment_of_compound_tape);
    }
}
