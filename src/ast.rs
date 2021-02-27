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

// #[derive(Debug, PartialEq, Clone)]
// pub enum UnaryOperator {
//   Negative,
// }
// impl fmt::Display for UnaryOperator {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{:?}", self)
//   }
// }
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  Mod,
}
impl fmt::Display for BinaryOperator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  Identifier(String),
  Integer(i32),
  // Unary {
  //   operator: UnaryOperator,
  //   expression: Box<Expression>,
  // },
  Binary {
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
  },
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Identifier(i) => write!(f, "{}", i)?,
      Expression::Integer(n) => write!(f, "{}", n)?,
      // Expression::Unary {
      //   operator,
      //   expression,
      // } => write!(f, "{}{}", operator, expression)?,
      Expression::Binary {
        left,
        operator,
        right,
      } => write!(f, "{}{}{}", left, operator, right)?,
    }
    Ok(())
  }
}
