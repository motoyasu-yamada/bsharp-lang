use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum TokenKind {
  ILLEGAL,
  DEFAULT,
  EOF,
  EOL,
  COMMA,
  IDENT,
  INT,
  STRING,
  CONST,
  PRINT,
  LPAREN,
  RPAREN,
  PLUS,
  MINUS,
  ASTERISK,
  SLASH,
  ASSIGN,
  PERCENT,
}

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
