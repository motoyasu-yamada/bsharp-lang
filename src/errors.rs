use super::token::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Errors {
  TokenInvalid(Token),
}

impl fmt::Display for Errors {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Errors::TokenInvalid(value) => write!(f, "invalid token: {:?}", value),
    }
  }
}
