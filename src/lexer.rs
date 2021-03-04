use super::input_stream::InputStream;
use super::keywords::get_keyword;
use super::token::Token;
use super::token_kind::TokenKind;

pub struct Lexer<'a> {
  input_stream: InputStream<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(input_stream: InputStream<'a>) -> Self {
    Lexer { input_stream }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();
    let token;
    match self.input_stream.current() {
      b'\r' => {
        if self.input_stream.prefetch() == b'\n' {
          token = self.new_token_with_2(TokenKind::EOL)
        } else {
          token = self.new_token_with_1(TokenKind::EOL);
        }
      }
      b'\n' => token = self.new_token_with_1(TokenKind::EOL),
      b',' => token = self.new_token_with_1(TokenKind::COMMA),
      b'*' => token = self.new_token_with_1(TokenKind::ASTERISK),
      b'%' => token = self.new_token_with_1(TokenKind::PERCENT),
      b'/' => token = self.new_token_with_1(TokenKind::SLASH),
      b'+' => token = self.new_token_with_1(TokenKind::PLUS),
      b'-' => token = self.new_token_with_1(TokenKind::MINUS),
      b'(' => token = self.new_token_with_1(TokenKind::LPAREN),
      b')' => token = self.new_token_with_1(TokenKind::RPAREN),
      b'=' => token = self.new_token_with_1(TokenKind::ASSIGN),
      b'^' => token = self.new_token_with_1(TokenKind::HAT),
      b'<' => match self.input_stream.prefetch() {
        b'>' => token = self.new_token_with_2(TokenKind::NE),
        b'=' => token = self.new_token_with_2(TokenKind::LE),
        _ => token = self.new_token_with_1(TokenKind::LT),
      },
      b'>' => match self.input_stream.prefetch() {
        b'=' => token = self.new_token_with_2(TokenKind::GE),
        _ => token = self.new_token_with_1(TokenKind::GT),
      },
      b'"' => token = self.parse_string(),
      0 => token = self.new_token(TokenKind::EOF, String::from("")),
      c => {
        if Self::is_letter(&c) {
          let ident = self.read_identifier();
          let kind = get_keyword(&ident);
          return self.new_token(kind, ident);
        } else if Self::is_digit(&c) {
          let literal = self.read_number();
          return self.new_token(TokenKind::INT, literal);
        } else {
          token = self.new_token_with_1(TokenKind::ILLEGAL);
        }
      }
    };
    self.input_stream.next();
    token
  }

  fn skip_whitespace(&mut self) {
    loop {
      let c = self.input_stream.current();
      if !(c == b' ' || c == b'\t') {
        break;
      }
      self.input_stream.next();
    }
  }

  fn read_identifier(&mut self) -> String {
    self.input_stream.start_range();
    loop {
      let c = self.input_stream.current();
      if !(Self::is_digit(&c) || Self::is_letter(&c)) {
        break;
      }
      self.input_stream.next();
    }
    self.input_stream.range_to_string()
  }

  fn parse_string(&mut self) -> Token {
    self.input_stream.next();
    self.input_stream.start_range();
    loop {
      let c = self.input_stream.current();
      if c == b'"' {
        break;
      }
      if c == 0 {
        return self.new_token_with_1(TokenKind::ILLEGAL);
      }
      self.input_stream.next();
    }
    self.new_token_by_range(TokenKind::STRING)
  }

  fn read_number(&mut self) -> String {
    self.input_stream.start_range();
    loop {
      let c = self.input_stream.current();
      if !Self::is_digit(&c) {
        break;
      }
      self.input_stream.next();
    }
    self.input_stream.range_to_string()
  }

  fn is_letter(ch: &u8) -> bool {
    let ch = char::from(*ch);
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
  }

  fn is_digit(ch: &u8) -> bool {
    let ch = char::from(*ch);
    return '0' <= ch && ch <= '9';
  }

  fn new_token(&self, kind: TokenKind, value: String) -> Token {
    let (file_name, line, column) = self.input_stream.current_location();

    Token {
      kind,
      value,
      file_name,
      line,
      column,
    }
  }

  fn new_token_with_1(&mut self, kind: TokenKind) -> Token {
    let value = self.input_stream.current_to_string();
    self.new_token(kind, value)
  }

  fn new_token_with_2(&mut self, kind: TokenKind) -> Token {
    let value = self.input_stream.current_2_to_string();
    self.new_token(kind, value)
  }

  fn new_token_by_range(&mut self, kind: TokenKind) -> Token {
    let value = self.input_stream.range_to_string();
    self.new_token(kind, value)
  }
}
