use super::ast::{
  expression::Expression, operator::BinaryOperator, operator::UnaryOperator, program::Program,
  statement::Statement,
};
use super::context::Context;
use super::object::{Add, Object, RuntimeType, TypeOf};
use super::runtime_error::RuntimeError;
use log::debug;
use std::rc::Rc;

pub struct Executor {
  context: Context,
}

impl Executor {
  pub fn new() -> Self {
    return Executor {
      context: Context::new_root(),
    };
  }

  pub fn execute(&mut self, program: &Program) -> Result<Object, RuntimeError> {
    debug!("[Executor] >>>execute");
    for f in program.functions.iter() {
      self
        .context
        .declare_variable(&f.identifier, &Object::Function(Rc::new(f.clone())))?;
    }

    let mut r = Object::Undefined;
    for s in program.statements.iter() {
      r = self.execute_statement(s)?;
      debug!("Statement: {}", r);
    }
    debug!("[Executor] <<<execute: {}", r);
    Ok(r)
  }

  fn execute_statements(&mut self, statements: &Vec<Statement>) -> Result<Object, RuntimeError> {
    debug!("[Executor] >>>execute_statements");
    for s in statements {
      self.execute_statement(s)?;
    }
    debug!("[Executor] <<<execute_statements");
    Ok(Object::Undefined)
  }

  fn execute_statement(&mut self, statement: &Statement) -> Result<Object, RuntimeError> {
    match statement {
      Statement::Declaration {
        identifier,
        expression,
      } => self.execute_const_assignment(identifier, &expression),
      Statement::Assignment {
        identifier,
        expression,
      } => self.execute_assignment(identifier, &expression),
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
        self.context.declare_variable(loop_counter, &counter)?;
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

          counter = self.context.get_variable(loop_counter)?;
          counter = counter.add(1)?;
          self.context.set_variable(loop_counter, &counter)?;
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
    identifier: &String,
    arguments: &Vec<Expression>,
  ) -> Result<Object, RuntimeError> {
    debug!("[Executor] >>>execute_method: {}", identifier);
    let ret_variable_name = String::from("Ret");
    let mut evaluated_arguments: Vec<Object> = vec![];
    for a in arguments {
      let e = self.execute_expression(a)?;
      evaluated_arguments.push(e)
    }

    let r = match identifier.as_str() {
      "Print" => {
        for a in evaluated_arguments {
          print!("{}", a);
        }
        println!("");
        Object::Undefined
      }
      _ => match self.context.get_variable(identifier)? {
        Object::Function(f) => {
          self.context.new_stack();
          if evaluated_arguments.len() != f.arguments.len() {
            return Err(RuntimeError::ArgumentMismatch(identifier.to_string()));
          }
          for i in 0..evaluated_arguments.len() {
            let an = &f.arguments[i];
            let av = &evaluated_arguments[i];
            self.context.declare_variable(an, av)?;
          }
          self
            .context
            .declare_variable(&ret_variable_name, &Object::Undefined)?;
          debug!("[Context]\n {}", self.context);
          self.execute_statements(&f.statements)?;
          let r = self.context.get_variable(&ret_variable_name)?;
          self.context.pop_stack();
          r
        }
        value => {
          return Err(RuntimeError::NonFunctionObjectIsInvoked(
            identifier.to_string(),
            value,
          ))
        }
      },
    };
    debug!("[Context]\n {}", self.context);
    debug!("[Executor] <<<execute_method: {}: {}", identifier, r);
    Ok(r)
  }

  fn execute_const_assignment(
    &mut self,
    identifier: &String,
    expression: &Expression,
  ) -> Result<Object, RuntimeError> {
    let evaluated = self.execute_expression(expression)?;
    self.context.declare_variable(identifier, &evaluated)?;
    Ok(evaluated)
  }

  fn execute_assignment(
    &mut self,
    identifier: &String,
    expression: &Expression,
  ) -> Result<Object, RuntimeError> {
    let evaluated = self.execute_expression(expression)?;
    self.context.set_variable(identifier, &evaluated)?;
    Ok(evaluated)
  }

  fn execute_expression(&mut self, expression: &Expression) -> Result<Object, RuntimeError> {
    match expression {
      Expression::Identifier(name) => self.context.get_variable(name),
      Expression::Integer(value) => Ok(Object::Integer(*value)),
      Expression::String(value) => Ok(Object::String(value.clone())),
      Expression::FunctionInvocation {
        identifier,
        arguments,
      } => self.execute_method(identifier, arguments),
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
