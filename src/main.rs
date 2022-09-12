mod error;
mod expression;
mod state;
mod statement;
mod token;
mod utils;

use state::*;
use std::env;
use token::{Token, TokenType};

const HEAD: &str = r#".globl main
main:
"#;
const TAIL: &str = r#"
ret"#;

fn expect_number(token: &Token) -> &String {
    if let TokenType::Int32(num) = token.get_type() {
        num
    } else {
        panic!();
    }
}

fn process(input: &str) -> SResult<()> {
    let mut state = State::new(input.to_string());
    let (_program, tokens) = state.parse()?;
    println!("{}", HEAD);
    for index in 0..tokens.len() {
        let token = &tokens[index];
        if token.is_eof() {
            break;
        }
        if index == 0 {
            println!("mov ${}, %rax", expect_number(token));
        } else if matches!(token.get_type(), TokenType::Minus) {
            println!("sub ${}, %rax", expect_number(&tokens[index + 1]));
        } else if matches!(token.get_type(), TokenType::Plus) {
            println!("add ${}, %rax", expect_number(&tokens[index + 1]));
        }
    }
    println!("{}", TAIL);
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
