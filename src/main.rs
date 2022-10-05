mod code_gen;
mod error;
mod expression;
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
    head!();
    code_gen::run(&program);
    tail!();
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
fn main_test() {
    let cwd = std::env::current_dir().unwrap();
    let shell_path = cwd.join("test.sh");
    let command = &format!("{}", shell_path.display());
    println!("{:?}", command);
    let mut command = std::process::Command::new(command);
    let output = command.output().unwrap();
    println!("stdout: {:?}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {:?}", String::from_utf8(output.stderr).unwrap());
    assert!(output.status.success())
}
