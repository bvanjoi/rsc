mod expression;
mod state;
mod statement;
mod token;
mod utils;

use state::*;
use std::env;

const HEAD: &str = r#".globl main
main:
"#;
const TAIL: &str = r#"
ret"#;

fn process(input: &str) -> SResult<String> {
    let mut state = State::new(input.to_string());
    let _program = state.parse()?;
    Ok(format!("{HEAD}{}{TAIL}", state.p.join("\n")))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err(format!("{:?} invalid number of arguments", args))
    } else {
        let assemble = process(&args[1]).unwrap();
        println!("{}", assemble);
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
