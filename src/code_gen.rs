use crate::{
    ast::{
        AddrExpr, AssignExpr, BinaryExpr, DerefExpr, Expr, IdentExpr, Int32Lit, LeftVal, Lit,
        UnaryExpr,
    },
    ast::{
        BinaryOp, BlockStmt, EmptyStmt, ExprStmt, ForStmt, IfStmt, Program, ReturnStmt, Stmt,
        WhileStmt,
    },
    head, pop, push, tail,
    token::TokenType,
};

pub fn run(program: &Program, context: Context) {
    let mut context = context;
    for stmt in &program.body {
        context.statement(stmt);
    }

    println!("{}", head!(program.stack_size));
    for code in &context.code {
        println!("{}", code);
    }
    println!("{}", tail!());
}

type Assemble = String;

type Code = Vec<Assemble>;

pub struct Context {
    /// use for block jump, such as `if-else`, `for-loop`.
    count: usize,
    code: Code,
    stack_size: usize,
}

impl Context {
    pub fn new(stack_size: usize) -> Self {
        Self {
            count: 0,
            code: Default::default(),
            stack_size,
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
        self.code.push(format!(".L.begin.{}:", c));
        self.expression(&stmt.test);
        self.code.push(format!("cmp $0, %rax"));
        self.code.push(format!("je .L.end.{}", c));
        self.statement(&stmt.body);
        self.code.push(format!("jmp .L.begin.{}", c));
        self.code.push(format!(".L.end.{}:", c));
    }

    fn for_statement(&mut self, stmt: &ForStmt) {
        let c = self.count();
        if let Some(init) = &stmt.init {
            self.expression(init);
        }
        self.code.push(format!(".L.begin.{}:", c));
        if let Some(test) = &stmt.test {
            self.expression(test);
            self.code.push(format!("cmp $0, %rax"));
            self.code.push(format!("je .L.end.{}", c));
        }
        self.statement(&stmt.body);
        if let Some(update) = &stmt.update {
            self.expression(update);
        }
        self.code.push(format!("jmp .L.begin.{}", c));
        self.code.push(format!(".L.end.{}:", c));
    }

    fn if_statement(&mut self, stmt: &IfStmt) {
        let c = self.count();
        self.expression(&stmt.test);
        self.code.push(format!("cmp $0, %rax"));
        self.code.push(format!("je .L.else.{}", c));
        self.statement(&stmt.consequent);
        self.code.push(format!("jmp .L.end.{}", c));
        self.code.push(format!(".L.else.{}:", c));
        if let Some(alternate) = &stmt.alternate {
            self.statement(alternate)
        }
        self.code.push(format!(".L.end.{}:", c));
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

        self.code.push(format!("jmp .L.return"));
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
            Expr::Deref(deref) => self.deref_expression(deref),
            Expr::Addr(addr) => self.addr_expression(addr),
        }
    }

    fn deref_expression(&mut self, deref: &DerefExpr) {
        self.expression(&deref.argument);
        self.code.push(format!("mov (%rax), %rax"));
    }

    fn addr_expression(&mut self, addr: &AddrExpr) {
        self.expression(&addr.argument);
        self.code.pop();
    }

    fn get_ident_address(&mut self, expr: &IdentExpr) -> isize {
        -1 * ((self.stack_size - expr.offset * 8) as isize)
    }

    fn ident_expression(&mut self, expr: &IdentExpr) {
        let address = self.get_ident_address(expr);
        // lea: load effective address
        // (%rbp) + address -> %rax
        self.code.push(format!("lea {}(%rbp), %rax", address));
        self.code.push(format!("mov (%rax), %rax"));
    }

    fn assign_expression(&mut self, expr: &AssignExpr) {
        // left
        match &*expr.left {
            LeftVal::Ident(ident) => {
                let address = self.get_ident_address(ident);
                self.code.push(format!("lea {}(%rbp), %rax", address));
            }
            LeftVal::Deref(deref) => self.expression(&deref.argument),
        }

        // --
        self.code.push(push!());
        self.expression(&expr.right);
        self.code.push(pop!("%rdi"));
        // move the value of (%rdi) to %rax
        self.code.push(format!("mov %rax, (%rdi)"));
    }

    fn binary_expression(&mut self, expr: &BinaryExpr) {
        self.expression(&expr.right);
        self.code.push(push!());
        self.expression(&expr.left);
        self.code.push(pop!("%rdi"));

        use BinaryOp::*;
        match expr.op {
            Add => {
                self.code.push(format!("add %rdi, %rax"));
            }
            AddrAdd(ref pos) => {
                use crate::ast::BinaryAddrPos::*;
                match pos {
                    Left => {
                        self.code.push(format!("imul $8, %rdi"));
                        self.code.push(format!("add %rdi, %rax"));
                    }
                    Right => {
                        self.code.push(format!("imul $8, %rax"));
                        self.code.push(format!("add %rdi, %rax"));
                    }
                    _ => unreachable!(),
                }
            }
            Sub => {
                self.code.push(format!("sub %rdi, %rax"));
            }
            AddrSub(ref pos) => {
                use crate::ast::BinaryAddrPos::*;
                match pos {
                    Left => {
                        self.code.push(format!("imul $8, %rdi"));
                        self.code.push(format!("sub %rdi, %rax"));
                    }
                    Both => {
                        self.code.push(format!("sub %rdi, %rax"));
                        // remove offset
                        self.code.push(format!("mov $8, %rdi"));
                        self.code.push(format!("cqo"));
                        self.code.push(format!("idiv %rdi"));
                    }
                    _ => unreachable!(),
                }
            }
            Mul => {
                self.code.push(format!("imul %rdi, %rax"));
            }
            Div => {
                self.code.push(format!("cqo"));
                self.code.push(format!("idiv %rdi"));
            }
            Equal => {
                self.code.push(format!("cmp %rdi, %rax"));
                self.code.push(format!("sete %al"));
                self.code.push(format!("movzb %al, %rax"));
            }
            NotEqual => {
                self.code.push(format!("cmp %rdi, %rax"));
                self.code.push(format!("setne %al"));
                self.code.push(format!("movzb %al, %rax"));
            }
            Less => {
                self.code.push(format!("cmp %rdi, %rax"));
                self.code.push(format!("setl %al"));
                self.code.push(format!("movzb %al, %rax"));
            }
            LessEqual => {
                self.code.push(format!("cmp %rdi, %rax"));
                self.code.push(format!("setle %al"));
                self.code.push(format!("movzb %al, %rax"));
            }
            Great => {
                self.code.push(format!("cmp %rdi, %rax"));
                self.code.push(format!("setg %al"));
                self.code.push(format!("movzb %al, %rax"));
            }
            GreatEqual => {
                self.code.push(format!("cmp %rdi, %rax"));
                self.code.push(format!("setge %al"));
                self.code.push(format!("movzb %al, %rax"));
            }
        }
    }

    fn literal(&mut self, lit: &Lit) {
        match lit {
            Lit::Int32(lit) => self.int32_literal(lit),
        }
    }

    fn int32_literal(&mut self, lit: &Int32Lit) {
        self.code.push(format!("mov ${}, %rax", lit.num));
    }

    fn unary_expression(&mut self, expr: &UnaryExpr) {
        use TokenType::*;
        match expr.op {
            Plus => self.expression(&expr.argument),
            Minus => {
                self.expression(&expr.argument);
                self.code.push(format!("neg %rax"));
            }
            _ => unreachable!(),
        }
    }
}
