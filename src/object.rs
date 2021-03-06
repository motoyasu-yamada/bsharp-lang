use super::runtime_error::RuntimeError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RuntimeType {
  Integer,
  Boolean,
  String,
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

#[derive(Debug, Clone)]
pub enum Object {
  Undefined,
  Integer(i32),
  Boolean(bool),
  String(String),
}
impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Object::Undefined => write!(f, "Undefined"),
      Object::Integer(v) => write!(f, "Integer({})", v),
      Object::Boolean(b) => write!(f, "Boolean({})", b),
      Object::String(s) => write!(f, "String(\"{}\")", s),
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
