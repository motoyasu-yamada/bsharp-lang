use super::statement::Statement;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
  pub identifier: String,
  pub arguments: Vec<String>,
  pub statements: Vec<Statement>,
}

impl fmt::Display for FunctionDeclaration {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(fmt, "Fn {} ({:?})", self.identifier, self.arguments)?;
    for s in self.statements.iter() {
      writeln!(fmt, "{}", s)?;
    }
    Ok(())
  }
}
