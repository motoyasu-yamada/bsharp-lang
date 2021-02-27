use super::ast::{Expression, Program, Statement};
use super::errors::Errors;
use log::debug;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
  Default,
  Null,
  InvalidObjectType,
  Integer(i32),
}
impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Object::Default => write!(f, "Default")?,
      Object::Null => write!(f, "Null")?,
      Object::Integer(v) => write!(f, "Integer({})", v)?,
      Object::InvalidObjectType => write!(f, "InvalidObjectType")?,
    }
    Ok(())
  }
}

pub struct Executor {
  variables: BTreeMap<String, Object>,
}

impl Executor {
  pub fn new() -> Executor {
    return Executor {
      variables: BTreeMap::new(),
    };
  }
  pub fn execute(&mut self, program: &Program) -> Result<Object, Errors> {
    let mut r = Object::Default;
    for s in program.statements.iter() {
      r = self.execute_statement(s)?;
      debug!("Statement: {}", r);
    }
    Ok(r)
  }

  pub fn set_variable(&mut self, name: String, value: Object) -> Object {
    debug!("set_variable: {}={}", name, value);
    self.variables.insert(name, value.clone());
    return value;
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

  fn execute_statement(&mut self, statement: &Statement) -> Result<Object, Errors> {
    match statement {
      Statement::ConstAssignment {
        identifier,
        expression,
      } => self.execute_const_assignment(identifier.to_string(), &expression),
      Statement::Print { arguments } => self.execute_print(arguments),
      Statement::Empty => Ok(Object::Null),
    }
  }

  fn execute_const_assignment(
    &mut self,
    identifier: String,
    expression: &Expression,
  ) -> Result<Object, Errors> {
    let evaluated = self.execute_expression(expression)?;
    self.set_variable(identifier.to_owned(), evaluated);
    Ok(Object::Null)
  }

  fn execute_expression(&mut self, expression: &Expression) -> Result<Object, Errors> {
    match expression {
      Expression::Identifier(name) => match self.get_variable(name) {
        Some(value) => Ok(value),
        _ => Ok(Object::Null),
      },
      Expression::Integer(value) => Ok(Object::Integer(*value)),
      Expression::Add { left, right } => {
        let l = self.execute_expression(&left)?;
        let r = self.execute_expression(&right)?;
        match (l, r) {
          (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l + r)),
          _ => Ok(Object::InvalidObjectType),
        }
      }
      Expression::Mul { left, right } => {
        let l = self.execute_expression(&left)?;
        let r = self.execute_expression(&right)?;
        match (l, r) {
          (Object::Integer(l), Object::Integer(r)) => Ok(Object::Integer(l * r)),
          _ => Ok(Object::InvalidObjectType),
        }
      }
    }
  }

  fn execute_print(&mut self, arguments: &Vec<Expression>) -> Result<Object, Errors> {
    let last = arguments.len() - 1;
    for (i, a) in arguments.iter().enumerate() {
      let v = self.execute_expression(a)?;
      let head = if i == 0 { "Print: " } else { "," };
      let trail = if i == last { "\n" } else { "" };
      print!("{}{}{}", head, v, trail);
    }
    Ok(Object::Null)
  }
}
