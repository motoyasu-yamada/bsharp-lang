use std::fmt;

pub mod expression;
pub mod program;
pub mod statement;

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
  NEGATIVE,
  POSITIVE,
  NOT,
}
impl fmt::Display for UnaryOperator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
  ADD,
  SUB,
  MUL,
  DIV,
  MOD,
  AND,
  OR,
  XOR,
  EXPOTENTIAL,
  EQ,
  NE,
  GT,
  GE,
  LT,
  LE,
}
impl fmt::Display for BinaryOperator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
