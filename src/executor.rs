use super::ast::{BinaryOperator, Expression, Program, Statement};
use super::errors::Errors;
use super::object::Object;
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
      Expression::Binary {
        left,
        operator,
        right,
      } => {
        let l = self.execute_expression(&left)?;
        let r = self.execute_expression(&right)?;
        match (l, r) {
          (Object::Integer(l), Object::Integer(r)) => match operator {
            BinaryOperator::Add => Ok(Object::Integer(l + r)),
            BinaryOperator::Sub => Ok(Object::Integer(l - r)),
            BinaryOperator::Mul => Ok(Object::Integer(l * r)),
            BinaryOperator::Div => Ok(Object::Integer(l / r)),
            BinaryOperator::Mod => Ok(Object::Integer(l % r)),
          },
          _ => Ok(Object::InvalidObjectType),
        }
      } // Expression::Unary {
        //   operator,
        //   expression,
        // } => {
        //   let evaluated = self.execute_expression(&expression)?;
        //   match evaluated {
        //     Object::Integer(n) => match operator {
        //       UnaryOperator::Negative => Ok(Object::Integer(-n)),
        //     },
        //     _ => Ok(Object::InvalidObjectType),
        //   }
        // }
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
