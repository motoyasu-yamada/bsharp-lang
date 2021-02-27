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
