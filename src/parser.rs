use super::ast::{
  expression::Expression, function::FunctionDeclaration, operator::BinaryOperator,
  operator::UnaryOperator, program::Program, statement::Statement,
};
use super::lexer::Lexer;
use super::parse_error::{ParseError, ParseErrorType};
use super::token::Token;
use super::token_kind::TokenKind;
use log::debug;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  current_token: Token,
  next_token: Token,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer<'a>) -> Self {
    let current_token = lexer.next_token();
    let next_token = lexer.next_token();
    return Parser {
      lexer,
      current_token,
      next_token,
    };
  }

  pub fn parse_program(&mut self) -> Result<Program, ParseError> {
    debug!(">>> parse_program");
    let mut statements: Vec<Statement> = vec![];
    let mut functions: Vec<FunctionDeclaration> = vec![];
    loop {
      match self.current_token.kind {
        TokenKind::EOF => break,
        TokenKind::FUNCTION => {
          let f = self.parse_function_declaration()?;
          functions.push(f);
        }
        _ => {
          let mut add = self.parse_statements(|k| match *k {
            TokenKind::EOF => true,
            TokenKind::FUNCTION => true,
            _ => false,
          })?;
          statements.append(&mut add);
        }
      }
    }
    let program = Program {
      statements,
      functions,
    };
    debug!("<<< parse_program: {}", program);
    Ok(program)
  }

  fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ParseError> {
    debug!(">>> parse_function_declaration {:?}", self.current_token);
    let to_stop: fn(&TokenKind) -> bool = |k| *k == TokenKind::END;
    if self.current_token.kind != TokenKind::FUNCTION {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected Fn or Function, but {}", self.current_token.kind),
      ));
    }

    self.next_token();
    if self.current_token.kind != TokenKind::IDENT {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected Ident, but {}", self.current_token.kind),
      ));
    }
    let identifier = self.current_token.value.clone();
    self.next_token();
    if self.current_token.kind != TokenKind::LPAREN {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected LPAREN, but {}", self.current_token.kind),
      ));
    }
    self.next_token();
    let mut arguments: Vec<String> = vec![];
    loop {
      if self.current_token.kind == TokenKind::RPAREN {
        break;
      }
      if self.current_token.kind != TokenKind::IDENT {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected Ident, but {}", self.current_token.kind),
        ));
      }
      let argument = self.current_token.value.clone();
      arguments.push(argument);
      self.next_token();
      if self.current_token.kind == TokenKind::RPAREN {
        continue;
      }
      if self.current_token.kind != TokenKind::COMMA {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected COMMA, but {}", self.current_token.kind),
        ));
      }
      self.next_token();
    }
    self.next_token();
    if self.current_token.kind != TokenKind::EOL {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected EOL, but {}", self.current_token.kind),
      ));
    }
    self.next_token();
    let statements: Vec<Statement> = self.parse_statements(to_stop)?;
    if self.current_token.kind != TokenKind::END {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected End, but {}", self.current_token.kind),
      ));
    }
    self.next_token();

    let f = FunctionDeclaration {
      identifier,
      arguments,
      statements,
    };
    debug!("<<< parse_function_declaration {:?}", f);
    Ok(f)
  }

  fn parse_statements(
    &mut self,
    to_stop: fn(&TokenKind) -> bool,
  ) -> Result<Vec<Statement>, ParseError> {
    let mut statements: Vec<Statement> = vec![];
    loop {
      debug!("parse_statements:loop {:?}", self.current_token);
      if to_stop(&self.current_token.kind) {
        debug!(
          "parse_statements: stopeed with: {}",
          self.current_token.kind
        );
        break;
      }
      let s = self.parse_statement()?;
      statements.push(s);
      let k = self.current_token.kind;
      if !(k == TokenKind::EOL || to_stop(&k)) {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected an end keyword of statement, but {}", k),
        ));
      }
      self.next_token();
    }
    return Ok(statements);
  }

  fn parse_statement(&mut self) -> Result<Statement, ParseError> {
    debug!(">>> parse_statement {}", self.current_token.kind);
    let s = match self.current_token.kind {
      TokenKind::IF => self.parse_if_statement()?,
      TokenKind::FOR => self.parse_for_statement()?,
      TokenKind::DIM => self.parse_const_assignment_statement()?,
      TokenKind::CONST => self.parse_const_assignment_statement()?,
      TokenKind::EOL => Statement::Empty,
      TokenKind::RETURN => self.parse_return_statement()?,
      _ => self.parse_expression_statement()?,
    };
    debug!("<<< parse_statement {}", s);
    Ok(s)
  }

  fn parse_return_statement(&mut self) -> Result<Statement,ParseError> {
    debug!(">>> parse_return_statement");
    self.next_token();
    let expression = self.parse_expression()?;
    if self.current_token.kind != TokenKind::EOL {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected EOL, but {}", self.current_token.kind),
      ));
    }
    debug!("<<< parse_return_statement {}", expression);
    Ok(Statement::ReturnStatement { expression })
  }

  fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
    debug!(">>> parse_if_statement");
    let mut if_blocks: Vec<(Expression, Vec<Statement>)> = vec![];
    let mut else_statements: Vec<Statement> = vec![];
    let to_stop: fn(&TokenKind) -> bool = |k| *k == TokenKind::ELSE || *k == TokenKind::END;

    {
      if self.current_token.kind != TokenKind::IF {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected IF, but {}", self.current_token.kind),
        ));
      }
      self.next_token();
      
      debug!(">>> condition");
      let condition = self.parse_expression()?;
      debug!("<<< condition");
      debug!(">>> then_block");
      if self.current_token.kind != TokenKind::THEN {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected THEN, but {}", self.current_token.kind),
        ));
      }
      self.next_token();
      if self.current_token.kind != TokenKind::EOL {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected EOL, but {}", self.current_token.kind),
        ));
      }
      self.next_token();
      let statements: Vec<Statement> = self.parse_statements(to_stop)?;
      debug!("<<< then_block");
      if_blocks.push((condition, statements));
    }

    loop {
      debug!("*** LOOP {}", self.current_token.kind);
      if self.current_token.kind == TokenKind::END {
        debug!("*** END");
        self.next_token();
        if self.current_token.kind != TokenKind::IF {
          return Err(self.raise_error(
            ParseErrorType::InvalidToken,
            format!("Expected IF, but {}", self.current_token.kind),
          ));
        }
        debug!("*** IF");
        self.next_token();
        debug!("*** BREAK");
        break;
      }
      if self.current_token.kind != TokenKind::ELSE {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Expected ELSE, but {}", self.current_token.kind),
        ));
      }
      if 0 < else_statements.len() {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!("Not expected ELSE, but {}", self.current_token.kind),
        ));
      }
      self.next_token();
      if self.current_token.kind == TokenKind::IF {
        self.next_token();
        let c = self.parse_expression()?;
        if self.current_token.kind != TokenKind::THEN {
          return Err(self.raise_error(
            ParseErrorType::InvalidToken,
            format!("Expected THEN, but {}", self.current_token.kind),
          ));
        }
        self.next_token();
        if self.current_token.kind != TokenKind::EOL {
          return Err(self.raise_error(
            ParseErrorType::InvalidToken,
            format!("Expected EOL, but {}", self.current_token.kind),
          ));
        }
        self.next_token();
        let statements: Vec<Statement> = self.parse_statements(to_stop)?;
        if_blocks.push((c, statements));
      } else {
        debug!(">>> ELSE");
        if self.current_token.kind != TokenKind::EOL {
          return Err(self.raise_error(
            ParseErrorType::InvalidToken,
            format!("Expected EOL, but {}", self.current_token.kind),
          ));
        }
        self.next_token();
        debug!("<<< ELSE {}", self.current_token.kind);
        else_statements = self.parse_statements(to_stop)?;
        debug!(">>>");
      }
    }

    Ok(Statement::IfStatement {
      if_blocks,
      else_statements,
    })
  }

  fn parse_for_statement(&mut self) -> Result<Statement, ParseError> {
    debug!(">>> parse_for_statement {}", self.current_token.kind);

    let to_stop: fn(&TokenKind) -> bool = |k| *k == TokenKind::NEXT;
    if self.current_token.kind != TokenKind::FOR {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected For, but {}", self.current_token.kind),
      ));
    }

    self.next_token();
    if self.current_token.kind != TokenKind::IDENT {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected Ident, but {}", self.current_token.kind),
      ));
    }
    let loop_counter = self.current_token.value.clone();
    self.next_token();
    debug!("*** loop_counter {},{:?}", loop_counter, self.current_token);
    if self.current_token.kind != TokenKind::ASSIGN {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected '=', but {}", self.current_token.kind),
      ));
    }
    self.next_token();
    let loop_counter_from = self.parse_expression()?;
    if self.current_token.kind != TokenKind::TO {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected To, but {}", self.current_token.kind),
      ));
    }
    self.next_token();
    let loop_counter_to = self.parse_expression()?;
    if self.current_token.kind != TokenKind::EOL {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected EOL, but {}", self.current_token.kind),
      ));
    }
    self.next_token();
    let block: Vec<Statement> = self.parse_statements(to_stop)?;
    if self.current_token.kind != TokenKind::NEXT {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected Next, but {}", self.current_token.kind),
      ));
    }
    self.next_token();
    Ok(Statement::ForStatement {
      loop_counter,
      loop_counter_from,
      loop_counter_to,
      block,
    })
  }

  /*
  - `ExpressionStatement`       ::= `Assignment` |
                                    `MethodInvocation`
  */
  fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
    debug!(">>> parse_expression_statement {}", self.current_token.kind);
    match self.parse_assignment()? {
      Some((identifier, expression)) => Ok(Statement::Assignment {
        identifier,
        expression,
      }),
      None => self.parse_method_invocation(),
    }
  }

  fn parse_const_assignment_statement(&mut self) -> Result<Statement, ParseError> {
    debug!(">>> parse_const_assignment_statement");
    self.next_token();
    match self.parse_assignment()? {
      Some((identifier, expression)) => Ok(Statement::Declaration {
        identifier,
        expression,
      }),
      None => Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected Const or Dim, but {}", self.current_token.kind),
      )),
    }
  }

  fn parse_assignment(&mut self) -> Result<Option<(String, Expression)>, ParseError> {
    if self.current_token.kind != TokenKind::IDENT {
      return Ok(None);
    }
    let identifier = self.current_token.value.clone();
    if self.next_token.kind != TokenKind::ASSIGN {
      return Ok(None);
    }
    self.next_token();
    self.next_token();
    let expression = self.parse_expression()?;

    return Ok(Some((identifier, expression)));
  }

  fn parse_function_invocation(&mut self) -> Result<Expression, ParseError> {
    debug!(">>> parse_function_invocation");
    if self.current_token.kind != TokenKind::IDENT {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected IDENT, but {}", self.current_token.kind),
      ));
    }
    let identifier = self.current_token.value.clone();
    self.next_token();
    if self.current_token.kind != TokenKind::LPAREN {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected LPAREN, but {}", self.current_token.kind),
      ));
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
          return Err(self.raise_error(
            ParseErrorType::InvalidToken,
            format!("Expected COMMA, but {}", self.current_token.kind),
          ));
        }
        self.next_token();
      }
    }
    let e = Expression::FunctionInvocation {
      identifier,
      arguments,
    };
    debug!("<<< parse_function_invocation: {}", e);
    return Ok(e);
  }

  fn parse_method_invocation(&mut self) -> Result<Statement, ParseError> {
    debug!(">>> parse_method_invocation");
    if self.current_token.kind != TokenKind::IDENT {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected IDENT, but {}", self.current_token.kind),
      ));
    }
    let identifier = self.current_token.value.clone();
    self.next_token();
    if self.current_token.kind != TokenKind::LPAREN {
      return Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected LPAREN, but {}", self.current_token.kind),
      ));
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
          return Err(self.raise_error(
            ParseErrorType::InvalidToken,
            format!("Expected COMMA, but {}", self.current_token.kind),
          ));
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
  fn parse_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(">>> parse_expression {}", self.current_token.kind);
    let e = {
      let e = self.parse_logical_or_expression()?;
      if self.current_token.kind != TokenKind::XOR {
        e
      } else {
        self.next_token();
        let right = self.parse_expression()?;
        self.binary_operation(&e, BinaryOperator::XOR, &right)
      }
    };
    debug!("<<< parse_expression {}", e);
    Ok(e)
  }

  fn parse_logical_or_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(
      ">>> parse_logical_or_expression {}",
      self.current_token.kind
    );
    let e = {
      let e = self.parse_logical_and_expression()?;
      if self.current_token.kind != TokenKind::OR {
        e
      } else {
        self.next_token();
        let right = self.parse_logical_or_expression()?;
        self.binary_operation(&e, BinaryOperator::OR, &right)
      }
    };
    debug!("<<< parse_logical_or_expression {}", e);
    Ok(e)
  }

  fn parse_logical_and_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(
      ">>> parse_logical_and_expression {}",
      self.current_token.kind
    );
    let e = self.parse_logical_not_expression()?;
    if self.current_token.kind != TokenKind::AND {
      return Ok(e);
    }
    self.next_token();
    let right = self.parse_logical_and_expression()?;
    Ok(self.binary_operation(&e, BinaryOperator::AND, &right))
  }

  fn parse_logical_not_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(
      ">>> parse_logical_not_expression {}",
      self.current_token.kind
    );
    if self.current_token.kind != TokenKind::NOT {
      return self.parse_equality_expression();
    }
    self.next_token();
    let e = self.parse_equality_expression()?;
    Ok(self.unary_operation(UnaryOperator::NOT, &e))
  }

  fn parse_equality_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(">>> parse_equality_expression {}", self.current_token.kind);
    let e = self.parse_additive_expression()?;
    let op;
    match self.current_token.kind {
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

  fn parse_additive_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(">>> parse_additive_expression {}", self.current_token.kind);
    let e = self.parse_multiplicative_expression()?;
    let op;
    match self.current_token.kind {
      TokenKind::PLUS => op = BinaryOperator::ADD,
      TokenKind::MINUS => op = BinaryOperator::SUB,
      _ => return Ok(e),
    }
    self.next_token();
    let right = self.parse_additive_expression()?;
    Ok(self.binary_operation(&e, op, &right))
  }

  fn parse_multiplicative_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(
      ">>> parse_multiplicative_expression {}",
      self.current_token.kind
    );
    let e = self.parse_unary_expression()?;
    let op;
    match self.current_token.kind {
      TokenKind::ASTERISK => op = BinaryOperator::MUL,
      TokenKind::SLASH => op = BinaryOperator::DIV,
      TokenKind::PERCENT => op = BinaryOperator::MOD,
      TokenKind::MOD => op = BinaryOperator::MOD,
      _ => return Ok(e),
    }
    self.next_token();
    let right = self.parse_unary_expression()?;
    Ok(self.binary_operation(&e, op, &right))
  }

  fn parse_unary_expression(&mut self) -> Result<Expression, ParseError> {
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

  fn parse_exponential_expression(&mut self) -> Result<Expression, ParseError> {
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

  fn parse_primary(&mut self) -> Result<Expression, ParseError> {
    debug!(">>> parse_primary {}", self.current_token.kind);
    let e = match self.current_token.kind {
      TokenKind::IDENT => match self.next_token.kind {
        TokenKind::LPAREN => self.parse_function_invocation()?,
        _ => Expression::Identifier(self.current_token.value.clone()),
      },
      TokenKind::INT => Expression::Integer(self.current_token.value.parse::<i32>().unwrap()),
      TokenKind::TRUE => Expression::Boolean(true),
      TokenKind::FALSE => Expression::Boolean(false),
      TokenKind::STRING => Expression::String(self.current_token.value.clone()),
      TokenKind::LPAREN => self.parse_grouped_expression()?,
      _ => {
        return Err(self.raise_error(
          ParseErrorType::InvalidToken,
          format!(
            "Expected IDENT, INT, LPAREN, but {}",
            self.current_token.kind
          ),
        ))
      }
    };
    self.next_token();
    debug!("<<< parse_primary {}", e);
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

  fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
    debug!(">>> parse_grouped_expression");
    self.next_token();
    let e = self.parse_expression()?;
    if self.current_token.kind == TokenKind::RPAREN {
      Ok(e)
    } else {
      Err(self.raise_error(
        ParseErrorType::InvalidToken,
        format!("Expected RPAREN, but {}", self.current_token.kind),
      ))
    }
  }

  fn raise_error(&mut self, error_type: ParseErrorType, error_message: String) -> ParseError {
    debug!(">>> raise_error: {},{}", error_type, error_message);
    ParseError {
      error_type,
      error_message,
      file_name: self.current_token.file_name.clone(),
      line: self.current_token.line,
      column: self.current_token.column,
    }
  }

  fn next_token(&mut self) {
    self.current_token = self.next_token.clone();
    self.next_token = self.lexer.next_token();
    debug!("next_token: {}", self.current_token.kind);
  }
}
