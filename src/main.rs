use std::fs::read_to_string;

pub mod core;
pub mod ast;
pub mod parse;

fn main() {
    let prog = read_to_string("prog.frac")
        .unwrap();

    let a = parse::prog_parser::prog(&prog).unwrap();

    println!("Result: {}", ast::Prog::print_names(&a.step_eval()));
}