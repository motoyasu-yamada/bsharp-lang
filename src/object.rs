use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RuntimeType {
  Integer,
  Boolean,
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

#[derive(Debug, Clone)]
pub enum Object {
  Undefined,
  Integer(i32),
  Boolean(bool),
}
impl fmt::Display for Object {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Object::Undefined => write!(f, "Undefined"),
      Object::Integer(v) => write!(f, "Integer({})", v),
      Object::Boolean(b) => write!(f, "Boolean({})", b),
    }
  }
}
impl TypeOf for Object {
  fn type_of(&self) -> RuntimeType {
    match self {
      Object::Undefined => RuntimeType::Undefined,
      Object::Integer(_) => RuntimeType::Integer,
      Object::Boolean(_) => RuntimeType::Boolean,
    }
  }
}
