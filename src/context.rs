use super::object::Object;
use super::runtime_error::RuntimeError;
use std::collections::BTreeMap;
use std::fmt;

type Variables = BTreeMap<String, Object>;

pub struct Context {
  stack: Vec<Variables>,
}
impl fmt::Display for Context {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    for variables in self.stack.iter() {
      for (name, value) in variables {
        writeln!(fmt, "{}={}", name, value)?;
      }
      writeln!(fmt, "----------------------------------------")?;
    }
    Ok(())
  }
}

impl Context {
  pub fn new_root() -> Context {
    return Context {
      stack: vec![BTreeMap::new()],
    };
  }

  pub fn new_stack(&mut self) {
    self.stack.push(BTreeMap::new());
  }

  pub fn pop_stack(&mut self) {
    if self.stack.len() == 1 {
      panic!("Expected child stack to pop, but here is root stack.");
    }
    self.stack.pop();
  }

  pub fn declare_variable(&mut self, name: &String, value: &Object) -> Result<(), RuntimeError> {
    let len = self.stack.len();
    let v = &mut self.stack[len - 1];
    if v.contains_key(name) {
      Err(RuntimeError::DuplicatedVariableDefinition(name.clone()))
    } else {
      v.insert(name.clone(), value.clone());
      Ok(())
    }
  }

  pub fn set_variable(&mut self, name: &String, value: &Object) -> Result<(), RuntimeError> {
    for i in (0..self.stack.len()).rev() {
      let v = &mut self.stack[i];
      if v.contains_key(name) {
        v.insert(name.clone(), value.clone());
        return Ok(());
      }
    }
    Err(RuntimeError::UndefinedVariable(name.clone()))
  }

  pub fn get_variable(&mut self, name: &String) -> Result<Object, RuntimeError> {
    for i in (0..self.stack.len()).rev() {
      let v = &mut self.stack[i];
      if v.contains_key(name) {
        if let Some(value) = v.get(name) {
          return Ok(value.clone());
        }
      }
    }
    Err(RuntimeError::UndefinedVariable(name.clone()))
  }
}
