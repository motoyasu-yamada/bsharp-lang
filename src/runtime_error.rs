use super::object::{Object, RuntimeType, TypeOf};
use std::fmt;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum RuntimeError {
  UndefinedVariable(String),
  DuplicatedVariableDefinition(String),
  NonFunctionObjectIsInvoked(String, Object),
  ArgumentMismatch(String),
  // UnsupportedFeature(String),
  UnknownMethod(String),
  TypeMismatch {
    expected: RuntimeType,
    actual: RuntimeType,
  },
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RuntimeError::UndefinedVariable(name) => {
        write!(f, "UndefinedVariable, {} is not defined", name)
      }
      RuntimeError::DuplicatedVariableDefinition(name) => write!(
        f,
        "DuplicatedVariableDefinition, {} is already defined.",
        name
      ),
      RuntimeError::ArgumentMismatch(name) => {
        write!(f, "Arguments passed to function \"{}\" is invalid", name)
      }
      RuntimeError::NonFunctionObjectIsInvoked(name, value) => write!(
        f,
        "NonFunctionObjectIsInvoked, {} is expected as Function, but actual is {}",
        name,
        value.type_of()
      ),
      RuntimeError::UnknownMethod(method) => write!(f, "UnknownMethod, {} is not defined.", method),
      // RuntimeError::UnsupportedFeature(feature) => {
      //   write!(f, "UnsupportedFeature, '{}' is not supported.", feature)
      // }
      RuntimeError::TypeMismatch { expected, actual } => write!(
        f,
        "Type mismatch, expected type is {}, but actual is {}.",
        expected, actual
      ),
    }
  }
}
