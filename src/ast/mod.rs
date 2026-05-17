//! Kay.ky AST - Abstract Syntax Tree nodes

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Get(Box<Expr>, String),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, FloorDiv, Modulo,
    Equal, NotEqual, Less, Greater,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate, Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Say(Expr),
    Assign(String, Expr),
    Define(String, Vec<String>, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Return(Option<Expr>),
    Block(Vec<Stmt>),
    Import(String),
}
