use super::ast::{BinaryOperator, Expression, Program, Statement};
use super::errors::Errors;
use super::lexer::Lexer;
use super::token::Token;
use super::token_kind::TokenKind;

use log::debug;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  current_token: Token,
  next_token: Token,
}

impl<'a> Parser<'a> {
  pub fn new(l: Lexer<'a>) -> Self {
    let mut p = Parser {
      lexer: l,
      current_token: Token {
        kind: TokenKind::DEFAULT,
        value: "value".to_string(),
      },
      next_token: Token {
        kind: TokenKind::DEFAULT,
        value: "value".to_string(),
      },
    };
    p.next_token();
    p.next_token();
    return p;
  }

  pub fn parse_program(&mut self) -> Result<Program, Errors> {
    debug!(">>> parse_program");
    let mut statements: Vec<Statement> = vec![];
    while self.current_token.kind != TokenKind::EOF {
      let s = self.parse_statement()?;
      debug!("Statement: {}", s);
      statements.push(s);
      let k = self.current_token.kind;
      if !(k == TokenKind::EOL || k == TokenKind::EOF) {
        debug!("*** {}", k);
        return Err(Errors::TokenInvalid(self.next_token.clone()));
      }
      self.next_token();
    }
    Ok(Program { statements })
  }

  fn parse_statement(&mut self) -> Result<Statement, Errors> {
    debug!(">>> parse_statement");
    let s;
    match self.current_token.kind {
      TokenKind::CONST => s = self.parse_const_assignment_statement()?,
      TokenKind::PRINT => s = self.parse_print_statement()?,
      _ => s = Statement::Empty,
    }
    Ok(s)
  }

  fn parse_const_assignment_statement(&mut self) -> Result<Statement, Errors> {
    debug!(">>> parse_const_assignment_statement");
    self.next_token();
    let identifier = self.current_token.value.clone();
    if !self.expect_next_token(TokenKind::ASSIGN) {
      return Err(Errors::TokenInvalid(self.next_token.clone()));
    }
    self.next_token();
    let expression = self.parse_expression()?;
    if self.current_token.kind != TokenKind::EOL {
      return Err(Errors::TokenInvalid(self.next_token.clone()));
    }
    let s = Statement::ConstAssignment {
      identifier,
      expression,
    };
    return Ok(s);
  }

  fn parse_expression(&mut self) -> Result<Expression, Errors> {
    debug!(">>> parse_expression {}", self.current_token.kind);
    let mut e = match self.current_token.kind {
      TokenKind::IDENT => Expression::Identifier(self.current_token.value.clone()),
      TokenKind::INT => Expression::Integer(self.parse_integer()?),
      TokenKind::LPAREN => self.parse_grouped_expression()?,
      // TokenKind::MINUS => {
      //   self.next_token();
      //   Expression::Unary {
      //     expression: Box::new(self.parse_expression()?),
      //     operator: UnaryOperator::Negative,
      //   }
      // }
      _ => return Err(Errors::TokenInvalid(self.current_token.clone())),
    };
    self.next_token();
    match self.current_token.kind {
      TokenKind::PLUS => e = self.parse_binary_operation(&e, BinaryOperator::Add)?,
      TokenKind::MINUS => e = self.parse_binary_operation(&e, BinaryOperator::Sub)?,
      TokenKind::ASTERISK => e = self.parse_binary_operation(&e, BinaryOperator::Mul)?,
      TokenKind::SLASH => e = self.parse_binary_operation(&e, BinaryOperator::Div)?,
      TokenKind::PERCENT => e = self.parse_binary_operation(&e, BinaryOperator::Mod)?,
      _ => e = e,
    }

    Ok(e)
  }

  fn parse_binary_operation(
    &mut self,
    left: &Expression,
    operator: BinaryOperator,
  ) -> Result<Expression, Errors> {
    self.next_token();
    let right = self.parse_expression()?;
    let e = Expression::Binary {
      left: Box::new(left.clone()),
      operator,
      right: Box::new(right),
    };
    Ok(e)
  }

  fn parse_grouped_expression(&mut self) -> Result<Expression, Errors> {
    debug!(">>> parse_grouped_expression");
    self.next_token();
    let e = self.parse_expression()?;
    if self.current_token.kind == TokenKind::RPAREN {
      Ok(e)
    } else {
      Err(Errors::TokenInvalid(self.current_token.clone()))
    }
  }

  fn parse_integer(&mut self) -> Result<i32, Errors> {
    Ok(self.current_token.value.parse::<i32>().unwrap())
  }

  fn parse_print_statement(&mut self) -> Result<Statement, Errors> {
    debug!(">>> parse_print_statement");
    self.next_token();
    let mut arguments: Vec<Expression> = vec![];
    loop {
      let e = self.parse_expression()?;
      arguments.push(e);
      let kind = self.current_token.kind;
      if kind != TokenKind::COMMA {
        break;
      }
    }
    return Ok(Statement::Print { arguments });
  }

  fn next_token(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.next_token();
    debug!("next_token: {}", self.current_token.kind);
  }

  fn expect_next_token(&mut self, expect: TokenKind) -> bool {
    if self.next_token.kind == expect {
      self.next_token();
      return true;
    } else {
      return false;
    }
  }
}
