use super::{BinaryOperator, UnaryOperator};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  Identifier(String),
  Integer(i32),
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
      Expression::Identifier(i) => write!(f, "'{}'", i)?,
      Expression::Integer(n) => write!(f, "[int]{}", n)?,
      Expression::Unary {
        operator,
        expression,
      } => write!(f, "({} {})", operator, expression)?,
      Expression::Binary {
        left,
        operator,
        right,
      } => write!(f, "({} {} {})", left, operator, right)?,
    }
    Ok(())
  }
}
