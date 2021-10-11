use crate::turing::{Action, Program};
use dot_writer::{Attributes, DotWriter};
use std::fmt::{Result, Write as FmtWrite};
use std::io::Write;

pub struct GraphWriter<'a> {
    output: &'a mut dyn Write,
}

impl<'a> GraphWriter<'a> {
    pub fn new(output: &'a mut dyn Write) -> Self {
        Self { output }
    }

    pub fn write(&mut self, program: &Program) -> Result {
        let mut writer = DotWriter::from(&mut self.output);
        let mut digraph = writer.digraph();
        for (key, action) in program {
            if let Action::Do {
                state,
                symbol,
                direction: _,
            } = action
            {
                let mut start: String = String::new();
                write!(start, "{}", key.state)?;
                let mut finish: String = String::new();
                write!(finish, "{}", state)?;
                let mut label = String::new();
                write!(label, "{}", symbol)?;
                digraph.edge(start, finish).attributes().set_label(&label);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn program_can_be_written() {
        let mut output: Vec<u8> = Vec::new();
        let mut writer = GraphWriter::new(&mut output);

        let program: Program = "1L1 0R1 1L0 1R0".parse().unwrap();
        writer.write(&program).unwrap();

        let graph = String::from_utf8(output);
        assert_eq!(graph, Ok("digraph {\n  0 -> 1 [label=\"1\"];\n  0 -> 1 [label=\"0\"];\n  1 -> 0 [label=\"1\"];\n  1 -> 0 [label=\"1\"];\n}\n".to_owned()))
    }
}
