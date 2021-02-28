use super::statement::Statement;
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
