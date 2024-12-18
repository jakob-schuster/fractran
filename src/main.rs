use std::fs::read_to_string;

use clap::Parser;

pub mod ast;
pub mod core;
pub mod parse;
pub mod util;

#[derive(Parser)]
struct Args {
    /// A path to a Fractran program to run.
    script: String,
}

fn main() {
    // parse the command line arguments
    let args = Args::parse();

    // get the string that represents the fractran program
    let prog_string = read_to_string(args.script).unwrap();

    // parse the program
    let prog = parse::prog_parser::prog(&prog_string).unwrap();

    // evaluate it, printing intermediate results, and print the final result
    let _ = &prog.eval();
}
