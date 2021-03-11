use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum TokenKind {
  ILLEGAL,
  EOF,
  EOL,
  COMMA,
  IDENT,
  INT,
  STRING,
  CONST,
  LPAREN,
  RPAREN,
  PLUS,
  MINUS,
  ASTERISK,
  SLASH,
  ASSIGN,
  PERCENT,
  HAT,
  MOD,
  AND,
  OR,
  XOR,
  NOT,
  IF,
  THEN,
  ELSE,
  END,
  FOR,
  NEXT,
  DIM,
  TO,
  // EQ,
  NE,
  GT,
  GE,
  LT,
  LE,
  FUNCTION,
}

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
