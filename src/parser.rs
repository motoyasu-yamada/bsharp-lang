use super::ast::{
  expression::Expression, program::Program, statement::Statement, BinaryOperator, UnaryOperator,
};
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
    debug!(">>> parse_statement {}", self.current_token.kind);
    let s = match self.current_token.kind {
      TokenKind::IF => self.parse_if_statement()?,
      TokenKind::FOR => self.parse_for_statement()?,
      TokenKind::DIM => self.parse_const_assignment_statement()?,
      TokenKind::CONST => self.parse_const_assignment_statement()?,
      TokenKind::EOL => Statement::Empty,
      _ => self.parse_expression_statement()?,
    };
    Ok(s)
  }

  fn parse_if_statement(&mut self) -> Result<Statement, Errors> {
    Err(Errors::Unsupported)
  }

  fn parse_for_statement(&mut self) -> Result<Statement, Errors> {
    Err(Errors::Unsupported)
  }

  /*
  - `ExpressionStatement`       ::= `Assignment` |
                                    `MethodInvocation`
  */
  fn parse_expression_statement(&mut self) -> Result<Statement, Errors> {
    debug!(">>> parse_expression_statement {}", self.current_token.kind);
    match self.parse_assignment()? {
      Some((identifier, expression)) => Ok(Statement::Assignment {
        identifier,
        expression,
      }),
      None => self.parse_method_invocation(),
    }
  }

  fn parse_const_assignment_statement(&mut self) -> Result<Statement, Errors> {
    debug!(">>> parse_const_assignment_statement");
    self.next_token();
    match self.parse_assignment()? {
      Some((identifier, expression)) => Ok(Statement::Declaration {
        identifier,
        expression,
      }),
      None => Err(Errors::Unsupported),
    }
  }

  fn parse_assignment(&mut self) -> Result<Option<(String, Expression)>, Errors> {
    if self.current_token.kind != TokenKind::IDENT {
      return Ok(None);
    }
    let identifier = self.current_token.value.clone();
    if self.next_token.kind != TokenKind::ASSIGN {
      return Ok(None);
    }
    self.next_token();
    let expression = self.parse_expression()?;

    return Ok(Some((identifier, expression)));
  }

  fn parse_method_invocation(&mut self) -> Result<Statement, Errors> {
    if self.current_token.kind != TokenKind::IDENT {
      return Err(Errors::TokenInvalid(self.current_token.clone()));
    }
    let identifier = self.current_token.value.clone();
    self.next_token();
    if self.current_token.kind != TokenKind::LPAREN {
      return Err(Errors::TokenInvalid(self.current_token.clone()));
    }
    self.next_token();
    let mut arguments: Vec<Expression> = vec![];
    if self.current_token.kind != TokenKind::RPAREN {
      loop {
        arguments.push(self.parse_expression()?);
        if self.current_token.kind == TokenKind::RPAREN {
          break;
        }
        if self.current_token.kind != TokenKind::COMMA {
          return Err(Errors::TokenInvalid(self.current_token.clone()));
        }
        self.next_token();
      }
    }
    self.next_token();
    return Ok(Statement::MethodInvocation {
      identifier,
      arguments,
    });
  }

  /*
  - `Expression`                ::= `LogicalXorExpression`
  - `LogicalXorExpression`      ::= `LogicalOrExpression` |
                                    `LogicalXorExpression` "Xor" `LogicalOrExpression`
  */
  fn parse_expression(&mut self) -> Result<Expression, Errors> {
    debug!(">>> parse_expression {}", self.current_token.kind);
    let e = self.parse_logical_or_expression()?;
    if self.next_token.kind != TokenKind::XOR {
      return Ok(e);
    }
    self.next_token();
    let right = self.parse_expression()?;
    Ok(self.binary_operation(&e, BinaryOperator::XOR, &right))
  }

  fn parse_logical_or_expression(&mut self) -> Result<Expression, Errors> {
    debug!(
      ">>> parse_logical_or_expression {}",
      self.current_token.kind
    );
    let e = self.parse_logical_and_expression()?;
    if self.next_token.kind != TokenKind::OR {
      return Ok(e);
    }
    self.next_token();
    let right = self.parse_logical_or_expression()?;
    Ok(self.binary_operation(&e, BinaryOperator::OR, &right))
  }

  fn parse_logical_and_expression(&mut self) -> Result<Expression, Errors> {
    debug!(
      ">>> parse_logical_and_expression {}",
      self.current_token.kind
    );
    let e = self.parse_logical_not_expression()?;
    if self.next_token.kind != TokenKind::AND {
      return Ok(e);
    }
    self.next_token();
    let right = self.parse_logical_and_expression()?;
    Ok(self.binary_operation(&e, BinaryOperator::AND, &right))
  }

  fn parse_logical_not_expression(&mut self) -> Result<Expression, Errors> {
    debug!(
      ">>> parse_logical_not_expression {}",
      self.current_token.kind
    );
    if self.next_token.kind != TokenKind::NOT {
      return self.parse_equality_expression();
    }
    self.next_token();
    let e = self.parse_equality_expression()?;
    Ok(self.unary_operation(UnaryOperator::NOT, &e))
  }

  fn parse_equality_expression(&mut self) -> Result<Expression, Errors> {
    debug!(">>> parse_equality_expression {}", self.current_token.kind);
    let e = self.parse_additive_expression()?;
    let op;
    match self.next_token.kind {
      TokenKind::ASSIGN => op = BinaryOperator::EQ,
      TokenKind::NE => op = BinaryOperator::NE,
      TokenKind::LT => op = BinaryOperator::LT,
      TokenKind::GT => op = BinaryOperator::GT,
      TokenKind::LE => op = BinaryOperator::LE,
      TokenKind::GE => op = BinaryOperator::GE,
      _ => return Ok(e),
    }
    self.next_token();
    let right = self.parse_equality_expression()?;
    Ok(self.binary_operation(&e, op, &right))
  }

  fn parse_additive_expression(&mut self) -> Result<Expression, Errors> {
    debug!(">>> parse_additive_expression {}", self.current_token.kind);
    let e = self.parse_multiplicative_expression()?;
    let op;
    match self.next_token.kind {
      TokenKind::PLUS => op = BinaryOperator::ADD,
      TokenKind::MINUS => op = BinaryOperator::SUB,
      _ => return Ok(e),
    }
    self.next_token();
    let right = self.parse_additive_expression()?;
    Ok(self.binary_operation(&e, op, &right))
  }

  fn parse_multiplicative_expression(&mut self) -> Result<Expression, Errors> {
    debug!(
      ">>> parse_multiplicative_expression {}",
      self.current_token.kind
    );
    let e = self.parse_unary_expression()?;
    let op;
    match self.next_token.kind {
      TokenKind::ASTERISK => op = BinaryOperator::MUL,
      TokenKind::SLASH => op = BinaryOperator::DIV,
      TokenKind::PERCENT => op = BinaryOperator::MOD,
      _ => return Ok(e),
    }
    self.next_token();
    let right = self.parse_unary_expression()?;
    Ok(self.binary_operation(&e, op, &right))
  }

  fn parse_unary_expression(&mut self) -> Result<Expression, Errors> {
    debug!(">>> parse_unary_expression {}", self.current_token.kind);
    let op;
    match self.current_token.kind {
      TokenKind::PLUS => op = UnaryOperator::POSITIVE,
      TokenKind::MINUS => op = UnaryOperator::NEGATIVE,
      _ => {
        return Ok(self.parse_exponential_expression()?);
      }
    }
    self.next_token();
    let e = self.parse_exponential_expression()?;
    Ok(self.unary_operation(op, &e))
  }

  fn parse_exponential_expression(&mut self) -> Result<Expression, Errors> {
    debug!(
      ">>> parse_exponential_expression {}",
      self.current_token.kind
    );
    let e = self.parse_primary()?;
    if self.next_token.kind != TokenKind::HAT {
      return Ok(e);
    }
    self.next_token();
    let right = self.parse_exponential_expression()?;
    Ok(self.binary_operation(&e, BinaryOperator::EXPOTENTIAL, &right))
  }

  fn parse_primary(&mut self) -> Result<Expression, Errors> {
    let e = match self.current_token.kind {
      TokenKind::IDENT => {
        let i = self.current_token.value.clone();
        self.next_token();
        Expression::Identifier(i)
      }
      TokenKind::INT => Expression::Integer(self.parse_integer()?),
      TokenKind::LPAREN => self.parse_grouped_expression()?,
      _ => return Err(Errors::TokenInvalid(self.current_token.clone())),
    };
    Ok(e)
  }

  fn binary_operation(
    &mut self,
    left: &Expression,
    operator: BinaryOperator,
    right: &Expression,
  ) -> Expression {
    Expression::Binary {
      left: Box::new(left.clone()),
      operator,
      right: Box::new(right.clone()),
    }
  }

  fn unary_operation(&mut self, operator: UnaryOperator, expression: &Expression) -> Expression {
    Expression::Unary {
      operator,
      expression: Box::new(expression.clone()),
    }
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

  fn next_token(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.next_token();
    debug!("next_token: {}", self.current_token.kind);
  }
}
