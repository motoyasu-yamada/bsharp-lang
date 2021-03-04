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
  IfStatement {
    if_blocks: Vec<(Expression, Vec<Statement>)>,
    else_statements: Vec<Statement>,
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
      Statement::IfStatement {
        if_blocks,
        else_statements,
      } => {
        for (condition, statements) in if_blocks {
          writeln!(f, "If {} Then", condition)?;
          for s in statements {
            writeln!(f, "{}", s)?;
          }
        }
        if 0 < else_statements.len() {
          writeln!(f, "Else")?;
          for s in else_statements {
            writeln!(f, "{}", s)?;
          }
        }
        writeln!(f, "End If")?;
      }
      Statement::Empty => write!(f, "<empty>")?,
    }
    Ok(())
  }
}
