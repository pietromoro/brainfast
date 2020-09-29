#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
  Add,       // +
  Sub,       // -
  Right,     // >
  Left,      // <
  Read,      // ,
  Write,     // .
  BeginLoop, // [
  EndLoop,   // ]
}

use self::Token::*;

fn main() {
  let tokens = tokenize("+-><,.[] > + - ABC , .. DE . F");
  println!("{:?}", tokens);
}

fn tokenize(input: &str) -> Vec<Token> {
  let mut tokens = Vec::<Token>::new();
  let mut chars = input.chars();

  while let Some(c) = chars.next() {
    match c {
      '+' => tokens.push(Add),
      '-' => tokens.push(Sub),
      '<' => tokens.push(Right),
      '>' => tokens.push(Left),
      ',' => tokens.push(Read),
      '.' => tokens.push(Write),
      '[' => tokens.push(BeginLoop),
      ']' => tokens.push(EndLoop),
      _ => {}
    }
  }

  tokens
}

fn generate(tokens: &[Token]) -> String {
  let mut output = String::new();

  for &token in tokens {
    match token {
      Add => {
        
      }
      Sub => {
        
      }
      Right => {
        
      }
      Left => {
        
      }
      Read => {
        
      }
      Write => {

      }
      BeginLoop => {
        output_source.push_str("while () {\n");
      }
      EndLoop => {
        output_source.push_str("}\n");
      }
    }
  }
}
