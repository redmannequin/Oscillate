
mod environment;
pub use environment::Environment;

pub mod error;
pub use error::EvalError;
pub use error::Result;

mod object;
pub use object::Object;

use std::rc::Rc;
use std::cell::RefCell;

use parser::Program;
use parser::Statement;
use parser::Expression;

use parser::expression::Real;

use parser::expression::Prefix;
use parser::expression::PrefixType;

use parser::expression::Infix;
use parser::expression::InfixEnum;

use parser::expression::Assign;
use parser::expression::Identifier;

pub fn run(program: &Program, env: Rc<RefCell<Environment>>) -> Result<Object> {
    let mut result = Object::Null;
    for statement in program {
        result = eval_stmt(statement, Rc::clone(&env))?;
    }
    Ok(result)
}


fn eval_stmt(statement: &Statement, env: Rc<RefCell<Environment>>) -> Result<Object> {
    match statement {
        Statement::Define(_) => Err(EvalError::Ops),
        Statement::Set(_) => Err(EvalError::Ops),
        Statement::Use(_) => Err(EvalError::Ops),
        Statement::Expression(exp) => eval_exp(exp, env)
    }
}

fn eval_exp(expression: &Expression, env: Rc<RefCell<Environment>>) -> Result<Object> {
    match expression {
        Expression::Assign(assign) => eval_assign_exp(assign, env),
        Expression::Real(num) => eval_number(num),
        Expression::Prefix(prefix) => eval_prefix_exp(prefix, env),
        Expression::Identifier(ident) => eval_ident(ident, env),
        Expression::Infix(infix) => eval_infix_exp(infix, env),
        _ => Err(EvalError::Ops)
    }
}

fn eval_number(number: &Real) -> Result<Object> {
    let num = number.get_number();
    let obj = Object::Real(num);
    Ok(obj)
}

fn eval_prefix_exp(prefix: &Prefix, env: Rc<RefCell<Environment>>) -> Result<Object> {
    let obj = eval_exp(prefix.expression.as_ref(), env)?;
    match prefix.prefix {
        PrefixType::Not => Err(EvalError::Ops),
        PrefixType::Minus => match obj {
            Object::Real(num) => Ok(Object::Real(-num)),
            _ =>  Err(EvalError::Ops)
        } 
    }
}

fn eval_infix_exp(infix: &Infix, env: Rc<RefCell<Environment>>) -> Result<Object> {
    let left_obj = eval_exp(infix.left_exp.as_ref(), Rc::clone(&env))?;
    let right_obj = eval_exp(infix.right_exp.as_ref(), env)?;

    match (left_obj, right_obj) {
        (Object::Real(left), Object::Real(right)) => {
            eval_integer_infix_exp(infix, left, right)
        },
        _ => Err(EvalError::Ops)
    }
}

fn eval_integer_infix_exp(infix: &Infix, left: f64, right: f64) -> Result<Object> {
    Ok(match infix.infix.0 {
        Some(InfixEnum::Equal) => Object::Bool(left == right),
        Some(InfixEnum::NotEqual) => Object::Bool(left != right),
        Some(InfixEnum::LessThan) => Object::Bool(left < right),
        Some(InfixEnum::GraterThan) => Object::Bool(left > right),
        Some(InfixEnum::Plus) => Object::Real(left + right),
        Some(InfixEnum::Minus) => Object::Real(left - right),
        Some(InfixEnum::Star) => Object::Real(left * right),
        Some(InfixEnum::Divide) => Object::Real(left / right),
        None => return Err(EvalError::Ops)
    })
}

fn eval_assign_exp(assign: &Assign, env: Rc<RefCell<Environment>>) -> Result<Object> {
    let result = eval_exp(assign.value.as_ref(), Rc::clone(&env))?;
    env.borrow_mut().set(assign.ident.name.as_ref(), result.clone());
    Ok(result)
}

fn eval_ident(ident: &Identifier, env: Rc<RefCell<Environment>>) -> Result<Object> {
    if let Some(obj) = env.borrow().get(ident.name.as_ref()) {
        return Ok(obj.borrow().clone());
    }
    Err(EvalError::IdentifierNotFound(ident.name.clone()))
}