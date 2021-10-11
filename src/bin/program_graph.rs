use busy_beaver::turing::Program;
use busy_beaver::graph::GraphWriter;
use std::env;
use std::fmt::Result;

fn main() -> Result {
    let args: Vec<String> = env::args().collect();
    let program: Program = args.get(1).and_then(|input| input.parse().ok()).unwrap();
    let mut output: Vec<u8> = Vec::new();
    let mut writer = GraphWriter::new(&mut output);
    writer.write(&program)?;
    let graph = String::from_utf8(output).unwrap();
    print!("{}", graph);
    Ok(())
}
