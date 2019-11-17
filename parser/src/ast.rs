///
/// Program
/// 
#[derive(Debug)]
pub struct Program {
    statments: Vec<Statment>
}

impl Program {
    pub fn new() -> Self { Self { statments: Vec::new() } }
    pub fn add_statment(&mut self, statment: Statment) { self.statments.push(statment) }
}

///
/// Statements
/// 
#[derive(Debug)]
pub enum Statment {
    Let(String, Expression),
    Expression(Expression)
}

///
/// Expression
/// 
#[derive(Debug)]
pub enum Expression {
    Start,
    Stop,
    Exit,

    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Array(Vec<Expression>),
}