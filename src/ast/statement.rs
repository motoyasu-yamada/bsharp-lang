use super::expression::Expression;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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
  ForStatement {
    loop_counter: String,
    loop_counter_from: Expression,
    loop_counter_to: Expression,
    block: Vec<Statement>,
  },
  ReturnStatement {
    expression: Expression,
  },
  Empty,
}
impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Declaration {
        identifier,
        expression,
      } => writeln!(f, "Const {} = {}", identifier, expression)?,
      Statement::Assignment {
        identifier,
        expression,
      } => writeln!(f, "{} = {}", identifier, expression)?,
      Statement::MethodInvocation {
        identifier,
        arguments,
      } => writeln!(f, "{}({:?})", identifier, arguments)?,
      Statement::ForStatement {
        loop_counter,
        loop_counter_from,
        loop_counter_to,
        block,
      } => {
        writeln!(
          f,
          "For {} = {} To {}",
          loop_counter, loop_counter_from, loop_counter_to
        )?;
        for s in block {
          writeln!(f, "{}", s)?;
        }
        writeln!(f, "Next")?;
      }
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
      Statement::ReturnStatement{expression} => writeln!(f, "Return {}", expression)?,
      Statement::Empty => write!(f, "<empty>")?,
    }
    Ok(())
  }
}
