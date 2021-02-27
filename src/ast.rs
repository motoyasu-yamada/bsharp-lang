use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Program {
  pub statements: Vec<Statement>,
}
impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for s in self.statements.iter() {
      writeln!(f, "{}", s)?;
    }
    Ok(())
  }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
  ConstAssignment {
    identifier: String,
    expression: Expression,
  },
  Print {
    arguments: Vec<Expression>,
  },
  Empty,
}
impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::ConstAssignment {
        identifier,
        expression,
      } => write!(f, "Const {} = {}", identifier, expression)?,
      Statement::Print { arguments } => write!(f, "Print {:?}", arguments)?,
      Statement::Empty => write!(f, "<empty>")?,
    }
    Ok(())
  }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
  Identifier(String),
  Integer(i32),
  Add {
    left: Box<Expression>,
    right: Box<Expression>,
  },
  Mul {
    left: Box<Expression>,
    right: Box<Expression>,
  },
}
impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Identifier(i) => write!(f, "{}", i)?,
      Expression::Integer(n) => write!(f, "{}", n)?,
      Expression::Add { left, right } => write!(f, "{} + {}", left, right)?,
      Expression::Mul { left, right } => write!(f, "{} * {}", left, right)?,
    }
    Ok(())
  }
}
