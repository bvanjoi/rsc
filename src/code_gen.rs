use crate::{
    expression::{BinaryExpr, Expr, Int32Literal, Literal, UnaryExpr},
    pop, push,
    statement::{ExprStmt, Program, Stmt},
    token::TokenType,
};

pub fn run(program: &Program) {
    for stmt in &program.body {
        statement(stmt)
    }
}

fn statement(stmt: &Stmt) {
    match stmt {
        Stmt::Expr(stmt) => expression_statement(stmt),
    }
}

fn expression_statement(stmt: &ExprStmt) {
    expression(&stmt.expr)
}

fn expression(expr: &Expr) {
    match expr {
        Expr::Binary(bin) => binary_expression(bin),
        Expr::Literal(lit) => literal(lit),
        Expr::Unary(unary) => unary_expression(unary),
    }
}

fn binary_expression(expr: &BinaryExpr) {
    expression(&expr.right);
    push!();
    expression(&expr.left);
    pop!("%rdi");

    use TokenType::*;
    match expr.op {
        Plus => {
            println!("add %rdi, %rax");
        }
        Minus => {
            println!("sub %rdi, %rax");
        }
        Star => {
            println!("imul %rdi, %rax");
        }
        Slash => {
            println!("cqo");
            println!("idiv %rdi");
        }
        Equal => {
            println!("cmp %rdi, %rax");
            println!("sete %al");
            println!("movzb %al, %rax");
        }
        NotEqual => {
            println!("cmp %rdi, %rax");
            println!("setne %al");
            println!("movzb %al, %rax");
        }
        Less => {
            println!("cmp %rdi, %rax");
            println!("setl %al");
            println!("movzb %al, %rax");
        }
        LessEqual => {
            println!("cmp %rdi, %rax");
            println!("setle %al");
            println!("movzb %al, %rax");
        }
        Great => {
            println!("cmp %rdi, %rax");
            println!("setg %al");
            println!("movzb %al, %rax");
        }
        GreatEqual => {
            println!("cmp %rdi, %rax");
            println!("setge %al");
            println!("movzb %al, %rax");
        }
        _ => unreachable!(),
    }
}

fn literal(lit: &Literal) {
    match lit {
        Literal::Int32(lit) => int32_literal(lit),
    }
}

fn int32_literal(lit: &Int32Literal) {
    println!("mov ${}, %rax", lit.num);
}

fn unary_expression(expr: &UnaryExpr) {
    match expr.op {
        TokenType::Plus => expression(&expr.argument),
        TokenType::Minus => {
            expression(&expr.argument);
            println!("neg %rax");
        }
        _ => unreachable!(),
    }
}
