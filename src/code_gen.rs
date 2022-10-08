use std::collections::HashMap;

use crate::{
    expression::{AssignExpr, BinaryExpr, Expr, IdentExpr, Int32Literal, Literal, UnaryExpr},
    head, pop, push,
    statement::{
        BlockStmt, EmptyStmt, ExprStmt, ForStmt, IfStmt, Program, ReturnStmt, Stmt, WhileStmt,
    },
    tail,
    token::TokenType,
};

pub fn run(program: &Program, context: Context) {
    let mut context = context;
    head!();
    for stmt in &program.body {
        context.statement(stmt);
    }
    tail!();
}

type Address = isize;

pub struct Context {
    idents: HashMap<String, Address>,
    /// use for block jump, such as `if-else`, `for-loop`.
    count: usize,
}

impl Context {
    pub fn new() -> Self {
        Self {
            idents: Default::default(),
            count: 0,
        }
    }

    fn count(&mut self) -> usize {
        self.count += 1;
        self.count
    }

    fn statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(stmt) => self.expression_statement(stmt),
            Stmt::Return(stmt) => self.return_statement(stmt),
            Stmt::Block(stmt) => self.block_statement(stmt),
            Stmt::Empty(stmt) => self.empty_statement(stmt),
            Stmt::If(stmt) => self.if_statement(stmt),
            Stmt::For(stmt) => self.for_statement(stmt),
            Stmt::While(stmt) => self.while_statement(stmt),
        }
    }

    fn while_statement(&mut self, stmt: &WhileStmt) {
        let c = self.count();
        println!(".L.begin.{}:", c);
        self.expression(&stmt.test);
        println!("cmp $0, %rax");
        println!("je .L.end.{}", c);
        self.statement(&stmt.body);
        println!("jmp .L.begin.{}", c);
        println!(".L.end.{}:", c);
    }

    fn for_statement(&mut self, stmt: &ForStmt) {
        let c = self.count();
        if let Some(init) = &stmt.init {
            self.expression(init);
        }
        println!(".L.begin.{}:", c);
        if let Some(test) = &stmt.test {
            self.expression(test);
            println!("cmp $0, %rax");
            println!("je .L.end.{}", c);
        }
        self.statement(&stmt.body);
        if let Some(update) = &stmt.update {
            self.expression(update);
        }
        println!("jmp .L.begin.{}", c);
        println!(".L.end.{}:", c);
    }

    fn if_statement(&mut self, stmt: &IfStmt) {
        let c = self.count();
        self.expression(&stmt.test);
        println!("cmp $0, %rax");
        println!("je .L.else.{}", c);
        self.statement(&stmt.consequent);
        println!("jmp .L.end.{}", c);
        println!(".L.else.{}:", c);
        if let Some(alternate) = &stmt.alternate {
            self.statement(alternate)
        }
        println!(".L.end.{}:", c);
    }

    fn empty_statement(&mut self, _stmt: &EmptyStmt) {
        ()
    }

    fn block_statement(&mut self, stmt: &BlockStmt) {
        stmt.body.iter().for_each(|item| self.statement(item))
    }

    fn return_statement(&mut self, stmt: &ReturnStmt) {
        if let Some(expr) = &stmt.argument {
            self.expression(expr);
        }

        println!("jmp .L.return");
    }

    fn expression_statement(&mut self, stmt: &ExprStmt) {
        self.expression(&stmt.expr)
    }

    fn expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary(bin) => self.binary_expression(bin),
            Expr::Literal(lit) => self.literal(lit),
            Expr::Unary(unary) => self.unary_expression(unary),
            Expr::Assign(assign) => self.assign_expression(assign),
            Expr::Ident(ident) => self.ident_expression(ident),
        }
    }

    fn get_ident_address(&mut self, expr: &IdentExpr) -> isize {
        let size = self.idents.len() as isize;
        let key = *self.idents.entry(expr.name.clone()).or_insert(size + 1);
        key * 8
    }

    fn ident_expression(&mut self, expr: &IdentExpr) {
        let address = self.get_ident_address(expr);
        println!("lea {}(%rbp), %rax", -address);
        println!("mov (%rax), %rax");
    }

    fn assign_expression(&mut self, expr: &AssignExpr) {
        // left
        let address = self.get_ident_address(&expr.left);
        println!("lea {}(%rbp), %rax", -address);
        // --
        push!();
        self.expression(&expr.right);
        pop!("%rdi");
        println!("mov %rax, (%rdi)");
    }

    fn binary_expression(&mut self, expr: &BinaryExpr) {
        self.expression(&expr.right);
        push!();
        self.expression(&expr.left);
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

    fn literal(&mut self, lit: &Literal) {
        match lit {
            Literal::Int32(lit) => self.int32_literal(lit),
        }
    }

    fn int32_literal(&mut self, lit: &Int32Literal) {
        println!("mov ${}, %rax", lit.num);
    }

    fn unary_expression(&mut self, expr: &UnaryExpr) {
        match expr.op {
            TokenType::Plus => self.expression(&expr.argument),
            TokenType::Minus => {
                self.expression(&expr.argument);
                println!("neg %rax");
            }
            _ => unreachable!(),
        }
    }
}
