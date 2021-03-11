mod ast;
mod executor;
mod input_stream;
mod keywords;
mod lexer;
mod object;
mod parse_error;
mod parser;
mod runtime_error;
mod token;
mod token_kind;
extern crate log;
mod context;
use env_logger;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  env::set_var("RUST_LOG", "debug");
  env_logger::init();

  let args: Vec<String> = env::args().collect();
  println!("B# version 0.0");
  println!("{:?}", args);
  let src = &args[1];
  let mut f = File::open(src).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  println!("Sourcecode:\n\n{}", contents);
  let mut e = executor::Executor::new();
  let i = input_stream::InputStream::new(&contents, String::from(src));
  let l = lexer::Lexer::new(i);
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
