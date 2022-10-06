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

    match expr.op {
        TokenType::Plus => {
            println!("add %rdi, %rax");
        }
        TokenType::Minus => {
            println!("sub %rdi, %rax");
        }
        TokenType::Star => {
            println!("imul %rdi, %rax");
        }
        TokenType::Slash => {
            println!("cqo");
            println!("idiv %rdi");
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
