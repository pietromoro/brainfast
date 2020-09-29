fn main() {
    println!("Hello, world!");
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
