//! Create directed graphs from complete programs.
//!
//! One can create a [directed graph](https://en.wikipedia.org/wiki/Directed_graph) from a Turing Machine program by the following procedure.
//! * For each state in the program create a vertex.
//! * For each transition from a state `a` to state `b`, add an edge between the corresponding vertices.
//!
//! For example, the following code
//!
//! ```
//! use busy_beaver::{turing::CompleteProgram, graph::GraphWriter};
//! let program: CompleteProgram = "1L1 1R2 1R0 1L1 1R1 1LH".parse().expect("a complete program description");
//! let mut output: Vec<u8> = Vec::new();
//! let mut writer = GraphWriter::new(&mut output);
//! writer.write(&program).expect("writer to succeed");
//! let graph = String::from_utf8(output).expect("succesful conversion to String");
//! print!("{}", graph);
//! ```
//!
//! produces a Graphviz description of the Turing machine program mentioned in Rabo paper with the following graph
//!
//! ![A directed graph representation of Rabo champion](https://fifth-postulate.nl/busy-beaver/image/rado.svg)
//!
use crate::turing::{Action, CompleteProgram};
use dot_writer::{Attributes, DotWriter};
use std::fmt::{Result, Write as FmtWrite};
use std::io::Write;

/// A writer that writes a directed graph from a program.
pub struct GraphWriter<'a> {
    output: &'a mut dyn Write,
}

impl<'a> GraphWriter<'a> {
    /// Accepts an output to write to and creates a `GraphWriter`.
    pub fn new(output: &'a mut dyn Write) -> Self {
        Self { output }
    }

    /// Write a directed graph representation of the complete program to the output.
    pub fn write(&mut self, program: &CompleteProgram) -> Result {
        let mut writer = DotWriter::from(&mut self.output);
        let mut digraph = writer.digraph();
        for (key, action) in program {
            if let Action::Do {
                state,
                symbol: _,
                direction: _,
            } = action
            {
                let mut start: String = String::new();
                write!(start, "{}", key.state)?;
                let mut finish: String = String::new();
                write!(finish, "{}", state)?;
                let mut label = String::new();
                write!(label, "{}", key.symbol)?;
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

        let program: CompleteProgram = "1L1 0R1 1L0 1R0".parse().unwrap();
        writer.write(&program).unwrap();

        let graph = String::from_utf8(output);
        assert_eq!(graph, Ok("digraph {\n  0 -> 1 [label=\"0\"];\n  0 -> 1 [label=\"1\"];\n  1 -> 0 [label=\"0\"];\n  1 -> 0 [label=\"1\"];\n}\n".to_owned()))
    }
}
