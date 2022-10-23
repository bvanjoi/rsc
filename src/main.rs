mod ast;
mod code_gen;
mod error;
mod expression;
mod object;
mod p;
mod state;
mod statement;
mod token;
mod utils;

use state::*;
use std::env;

fn process(input: &str) -> SResult<()> {
    let mut state = State::new(input.to_string());
    let program = state.parse()?;
    let context = code_gen::Context::new(program.stack_size);
    code_gen::run(&program, context);
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(format!("{:?} invalid number of arguments", args))
    } else {
        process(&args[1]).unwrap();
        Ok(())
    }
}

#[test]
fn tttt() {
    process("{ &y-2+1; }").unwrap();
}
