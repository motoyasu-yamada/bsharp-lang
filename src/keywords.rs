use super::token_kind::TokenKind;

pub fn get_keyword(ident: &str) -> TokenKind {
  match ident {
    "Const" => TokenKind::CONST,
    "Dim" => TokenKind::DIM,
    "And" => TokenKind::AND,
    "Not" => TokenKind::NOT,
    "Or" => TokenKind::OR,
    "Xor" => TokenKind::XOR,
    "Mod" => TokenKind::MOD,
    "If" => TokenKind::IF,
    "Then" => TokenKind::THEN,
    "Else" => TokenKind::ELSE,
    "End" => TokenKind::END,
    "For" => TokenKind::FOR,
    "To" => TokenKind::TO,
    "Next" => TokenKind::NEXT,
    "Function" => TokenKind::FUNCTION,
    "Fn" => TokenKind::FUNCTION,
    "True" => TokenKind::TRUE,
    "False" => TokenKind::FALSE,   
    "Return" => TokenKind::RETURN, 
    _ => TokenKind::IDENT,
  }
}
