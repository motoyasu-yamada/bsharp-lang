use super::token_kind::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  pub kind: TokenKind,
  pub value: String,
}

// impl Token {}
