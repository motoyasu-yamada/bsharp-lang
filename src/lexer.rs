use super::keywords::get_keyword;
use super::token::Token;
use super::token_kind::TokenKind;

pub struct Lexer<'a> {
  input: &'a str,
  position: usize,
  read_position: usize,
  // current_line: usize,
  // current_column: usize,
  ch: u8,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut l = Lexer {
      input,
      position: 0,
      read_position: 0,
      ch: 0,
    };
    l.read_char();
    return l;
  }

  pub fn new_token(kind: TokenKind, ch: u8) -> Token {
    Token {
      kind,
      value: String::from_utf8(vec![ch]).unwrap(),
    }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    let token;
    match self.ch {
      b'\r' => {
        self.read_char();
        if self.ch == b'\n' {
          token = Token {
            kind: TokenKind::EOL,
            value: "\r\n".to_string(),
          }
        } else {
          return Self::new_token(TokenKind::EOL, b'\r');
        }
      }
      b'\n' => token = Self::new_token(TokenKind::EOL, self.ch),
      b'*' => token = Self::new_token(TokenKind::ASTERISK, self.ch),
      b'%' => token = Self::new_token(TokenKind::PERCENT, self.ch),
      b'/' => {
        token = Self::new_token(TokenKind::SLASH, self.ch);
      }
      b'+' => {
        token = Self::new_token(TokenKind::PLUS, self.ch);
      }
      b'-' => {
        token = Self::new_token(TokenKind::MINUS, self.ch);
      }
      b'=' => {
        token = Self::new_token(TokenKind::ASSIGN, self.ch);
      }
      b'(' => {
        token = Self::new_token(TokenKind::LPAREN, self.ch);
      }
      b')' => {
        token = Self::new_token(TokenKind::RPAREN, self.ch);
      }
      b'"' => {
        token = Token {
          kind: TokenKind::STRING,
          value: self.read_string(),
        }
      }
      0 => {
        token = Token {
          kind: TokenKind::EOF,
          value: String::from(""),
        }
      }
      _ => {
        if Self::is_letter(&self.ch) {
          let ident = self.read_identifier();
          let kind = get_keyword(&ident);
          token = Token { kind, value: ident };
          return token;
        } else if Self::is_digit(&self.ch) {
          token = Token {
            kind: TokenKind::INT,
            value: self.read_number(),
          };
          return token;
        } else {
          token = Self::new_token(TokenKind::ILLEGAL, self.ch);
        }
      }
    };
    self.read_char();
    return token;
  }

  fn skip_whitespace(&mut self) {
    while self.ch == b' ' || self.ch == b'\t' {
      self.read_char();
    }
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = 0;
    } else {
      self.ch = self.input.as_bytes()[self.read_position];
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn read_identifier(&mut self) -> String {
    let start = self.position;
    while Self::is_digit(&self.ch) || Self::is_letter(&self.ch) {
      self.read_char();
    }
    self.input.get(start..self.position).unwrap().to_string()
  }

  fn read_string(&mut self) -> String {
    let position = self.position + 1;
    loop {
      self.read_char();
      if self.ch == b'"' || self.ch == 0 {
        break;
      }
    }
    return self.input[position..self.position].to_string();
  }

  fn read_number(&mut self) -> String {
    let start = self.position;
    while Self::is_digit(&self.ch) {
      self.read_char();
    }
    self.input.get(start..self.position).unwrap().to_string()
  }

  fn is_letter(ch: &u8) -> bool {
    let ch = char::from(*ch);
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
  }

  fn is_digit(ch: &u8) -> bool {
    let ch = char::from(*ch);
    return '0' <= ch && ch <= '9';
  }
}
