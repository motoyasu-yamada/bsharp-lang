mod ast;
mod errors;
mod executor;
mod keywords;
mod lexer;
mod parser;
mod token;
mod token_kind;

extern crate log;
use env_logger;
use std::env;

fn main() {
  env::set_var("RUST_LOG", "debug");
  env_logger::init();
  println!("B# version 0.0");
  let line = r#"
Const a = 1 + 2
Const b = a * (a + 1)
Print b
  "#;
  let mut e = executor::Executor::new();
  let l = lexer::Lexer::new(&line);
  let mut parser = parser::Parser::new(l);
  let program = parser.parse_program();
  match program {
    Ok(p) => {
      println!("{:?}", p);
      let r = e.execute(&p);
      match r {
        Ok(r) => {
          println!("Result: {}", r);
        }
        Err(e) => {
          println!("Execution error: {}", e);
        }
      }
    }
    Err(e) => {
      println!("Compile error: {}", e);
    }
  }
}
