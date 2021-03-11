use super::function::FunctionDeclaration;
use super::statement::Statement;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Program {
  pub statements: Vec<Statement>,
  pub functions: Vec<FunctionDeclaration>,
}

impl fmt::Display for Program {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    for s in self.statements.iter() {
      writeln!(fmt, "{}", s)?;
    }
    let functions = &self.functions;
    for f in functions {
      writeln!(fmt, "fn {} ({:?})", f.identifier, f.arguments)?;
    }
    Ok(())
  }
}
