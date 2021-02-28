use super::expression::Expression;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Statement {
  Declaration {
    identifier: String,
    expression: Expression,
  },
  Assignment {
    identifier: String,
    expression: Expression,
  },
  MethodInvocation {
    identifier: String,
    arguments: Vec<Expression>,
  },
  Empty,
}
impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Declaration {
        identifier,
        expression,
      } => write!(f, "Const {} = {}", identifier, expression)?,
      Statement::Assignment {
        identifier,
        expression,
      } => write!(f, "{} = {}", identifier, expression)?,
      Statement::MethodInvocation {
        identifier,
        arguments,
      } => write!(f, "{}({:?})", identifier, arguments)?,
      Statement::Empty => write!(f, "<empty>")?,
    }
    Ok(())
  }
}
