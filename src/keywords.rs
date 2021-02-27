use super::token_kind::TokenKind;

pub fn get_keyword(ident: &str) -> TokenKind {
  match ident {
    "Const" => TokenKind::CONST,
    "Print" => TokenKind::PRINT,
    _ => TokenKind::IDENT,
  }
}
