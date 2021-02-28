use super::object::RuntimeType;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
  // Unknown,
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
      // RuntimeError::Unknown => write!(f, "Unknown"),
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
