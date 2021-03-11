use super::ast::function::FunctionDeclaration;
use super::runtime_error::RuntimeError;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum RuntimeType {
  Integer,
  Boolean,
  String,
  Function,
  Undefined,
}
impl fmt::Display for RuntimeType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self)
  }
}

pub trait TypeOf {
  fn type_of(&self) -> RuntimeType;
}

pub trait Add {
  fn add(&self, offset: i32) -> Result<Object, RuntimeError>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
  Undefined,
  Integer(i32),
  Boolean(bool),
  String(String),
  Function(Rc<FunctionDeclaration>),
}
impl fmt::Display for Object {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Object::Undefined => write!(fmt, "Undefined"),
      Object::Integer(v) => write!(fmt, "Integer({})", v),
      Object::Boolean(b) => write!(fmt, "Boolean({})", b),
      Object::String(s) => write!(fmt, "String(\"{}\")", s),
      Object::Function(f) => write!(fmt, "Function(\"{}\")", f),
    }
  }
}
impl TypeOf for Object {
  fn type_of(&self) -> RuntimeType {
    match self {
      Object::Undefined => RuntimeType::Undefined,
      Object::Integer(_) => RuntimeType::Integer,
      Object::Boolean(_) => RuntimeType::Boolean,
      Object::String(_) => RuntimeType::String,
      Object::Function(_) => RuntimeType::Function,
    }
  }
}
impl Add for Object {
  fn add(&self, offset: i32) -> Result<Object, RuntimeError> {
    let actual = self.type_of();
    match self {
      Object::Integer(n) => Ok(Object::Integer(n + offset)),
      _ => Err(RuntimeError::TypeMismatch {
        expected: RuntimeType::Integer,
        actual,
      }),
    }
  }
}
