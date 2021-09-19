use super::operator::{BinaryOperator, UnaryOperator};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  Boolean(bool),
  Identifier(String),
  String(String),
  Integer(i32),
  FunctionInvocation {
    identifier: String,
    arguments: Vec<Expression>,
  },
  Unary {
    operator: UnaryOperator,
    expression: Box<Expression>,
  },
  Binary {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
  },
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Boolean(b) => write!(f, "Expression::Boolean({})", b)?,
      Expression::Identifier(i) => write!(f, "Expression::Identifier({})", i)?,
      Expression::Integer(n) => write!(f, "Expression::Intger({})", n)?,
      Expression::String(s) => write!(f, "Expression::String(\"{}\")", s)?,
      Expression::FunctionInvocation {
        identifier,
        arguments,
      } => writeln!(f, "{}({:?})", identifier, arguments)?,
      Expression::Unary {
        operator,
        expression,
      } => write!(f, "Expression::Unary ({} {})", operator, expression)?,
      Expression::Binary {
        left,
        operator,
        right,
      } => write!(f, "Expression::Binary ({} {} {})", left, operator, right)?,
    }
    Ok(())
  }
}
