use super::ast::{
  expression::Expression, program::Program, statement::Statement, BinaryOperator, UnaryOperator,
};
use super::object::{Add, Object, RuntimeType, TypeOf};
use super::runtime_error::RuntimeError;
use log::debug;
use std::collections::BTreeMap;

pub struct Executor {
  variables: BTreeMap<String, Object>,
}

impl Executor {
  pub fn new() -> Executor {
    return Executor {
      variables: BTreeMap::new(),
    };
  }
  pub fn execute(&mut self, program: &Program) -> Result<Object, RuntimeError> {
    let mut r = Object::Undefined;
    for s in program.statements.iter() {
      r = self.execute_statement(s)?;
      debug!("Statement: {}", r);
    }
    Ok(r)
  }

  pub fn set_variable(&mut self, name: String, value: &Object) {
    debug!("set_variable: {}={}", name, value);
    self.variables.insert(name, value.clone());
  }

  pub fn get_variable(&mut self, name: &str) -> Option<Object> {
    match self.variables.get(name) {
      Some(value) => {
        debug!("get_variable: {}: {}", name, value);
        Some(value.clone())
      }
      None => {
        debug!("get_variable: {}: None", name);
        None
      }
    }
  }

  fn execute_statements(&mut self, statements: &Vec<Statement>) -> Result<Object, RuntimeError> {
    for s in statements {
      self.execute_statement(s)?;
    }
    Ok(Object::Undefined)
  }

  fn execute_statement(&mut self, statement: &Statement) -> Result<Object, RuntimeError> {
    match statement {
      Statement::Declaration {
        identifier,
        expression,
      } => self.execute_const_assignment(identifier.to_string(), &expression),
      Statement::Assignment {
        identifier,
        expression,
      } => self.execute_const_assignment(identifier.to_string(), &expression),
      Statement::MethodInvocation {
        identifier,
        arguments,
      } => self.execute_method(identifier, &arguments),
      Statement::ForStatement {
        loop_counter,
        loop_counter_from,
        loop_counter_to,
        block,
      } => {
        let mut counter = self.execute_expression(loop_counter_from)?;
        self.set_variable(loop_counter.to_owned(), &counter);
        loop {
          let to_value = self.execute_expression(loop_counter_to)?;
          let exit = match (counter, to_value) {
            (Object::Integer(l), Object::Integer(r)) => r < l,
            _ => {
              return Err(RuntimeError::TypeMismatch {
                expected: RuntimeType::Integer,
                actual: RuntimeType::Integer, // to_value.type_of(),
              });
            }
          };
          if exit {
            break;
          }
          self.execute_statements(block)?;

          counter = match self.get_variable(loop_counter) {
            Some(v) => v,
            None => return Err(RuntimeError::UndefinedVariable(loop_counter.to_string())),
          };
          counter = counter.add(1)?;
          self.set_variable(loop_counter.to_owned(), &counter);
        }
        return Ok(Object::Undefined);
      }
      Statement::IfStatement {
        if_blocks,
        else_statements,
      } => {
        for (c, b) in if_blocks {
          let e = self.execute_expression(c)?;
          match e {
            Object::Boolean(true) => {
              for s in b {
                self.execute_statement(s)?;
              }
              return Ok(Object::Undefined);
            }
            Object::Boolean(false) => {}
            a => {
              return Err(RuntimeError::TypeMismatch {
                expected: RuntimeType::Boolean,
                actual: a.type_of(),
              })
            }
          }
        }
        for s in else_statements {
          self.execute_statement(s)?;
        }
        return Ok(Object::Undefined);
      }
      Statement::Empty => Ok(Object::Undefined),
    }
  }

  fn execute_method(
    &mut self,
    identifier: &str,
    arguments: &Vec<Expression>,
  ) -> Result<Object, RuntimeError> {
    match identifier {
      "Print" => {
        for a in arguments {
          let evaluated = self.execute_expression(a)?;
          println!("{}", evaluated);
        }
        Ok(Object::Undefined)
      }
      _ => Err(RuntimeError::UnknownMethod(identifier.to_string())),
    }
  }

  fn execute_const_assignment(
    &mut self,
    identifier: String,
    expression: &Expression,
  ) -> Result<Object, RuntimeError> {
    let evaluated = self.execute_expression(expression)?;
    self.set_variable(identifier.to_owned(), &evaluated);
    Ok(evaluated)
  }

  fn execute_expression(&mut self, expression: &Expression) -> Result<Object, RuntimeError> {
    match expression {
      Expression::Identifier(name) => match self.get_variable(name) {
        Some(value) => Ok(value),
        _ => Ok(Object::Undefined),
      },
      Expression::Integer(value) => Ok(Object::Integer(*value)),
      Expression::String(value) => Ok(Object::String(value.clone())),
      Expression::Binary {
        left,
        operator,
        right,
      } => {
        let l = self.execute_expression(&left)?;
        let r = self.execute_expression(&right)?;
        match (l, r) {
          (Object::Integer(l), Object::Integer(r)) => match operator {
            BinaryOperator::ADD => Ok(Object::Integer(l + r)),
            BinaryOperator::SUB => Ok(Object::Integer(l - r)),
            BinaryOperator::MUL => Ok(Object::Integer(l * r)),
            BinaryOperator::DIV => Ok(Object::Integer(l / r)),
            BinaryOperator::MOD => Ok(Object::Integer(l % r)),
            BinaryOperator::EQ => Ok(Object::Boolean(l == r)),
            BinaryOperator::NE => Ok(Object::Boolean(l != r)),
            BinaryOperator::GT => Ok(Object::Boolean(l > r)),
            BinaryOperator::LT => Ok(Object::Boolean(l < r)),
            BinaryOperator::LE => Ok(Object::Boolean(l >= r)),
            BinaryOperator::GE => Ok(Object::Boolean(l <= r)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Boolean,
            }),
          },
          (Object::Boolean(l), Object::Boolean(r)) => match operator {
            BinaryOperator::AND => Ok(Object::Boolean(l && r)),
            BinaryOperator::XOR => Ok(Object::Boolean(l || r)),
            BinaryOperator::OR => Ok(Object::Boolean(l || r)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Boolean,
              actual: RuntimeType::Integer,
            }),
          },
          (l, r) => Err(RuntimeError::TypeMismatch {
            expected: l.type_of(),
            actual: r.type_of(),
          }),
        }
      }
      Expression::Unary {
        operator,
        expression,
      } => {
        let evaluated = self.execute_expression(&expression)?;
        match operator {
          UnaryOperator::NEGATIVE => match evaluated {
            Object::Integer(n) => Ok(Object::Integer(-n)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Integer,
            }),
          },
          UnaryOperator::POSITIVE => match evaluated {
            Object::Integer(n) => Ok(Object::Integer(n)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Integer,
            }),
          },
          UnaryOperator::NOT => match evaluated {
            Object::Boolean(n) => Ok(Object::Boolean(!n)),
            _ => Err(RuntimeError::TypeMismatch {
              expected: RuntimeType::Integer,
              actual: RuntimeType::Integer,
            }),
          },
        }
      }
    }
  }
}
