use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseErrorType {
  InvalidToken,
  Unsupported,
}
impl fmt::Display for ParseErrorType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
  pub error_type: ParseErrorType,
  pub error_message: String,
  pub file_name: String,
  pub line: usize,
  pub column: usize,
}
impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Error: {}", self.error_type)?;
    writeln!(
      f,
      "line: {}, column: {}, at {}",
      self.line + 1,
      self.column + 1,
      self.file_name
    )?;
    writeln!(f, "{}", self.error_message)?;
    Ok(())
  }
}
