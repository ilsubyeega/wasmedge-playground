use std::env;

fn main() {
  println!("Hello World!");
  for argument in env::args().skip(1) {
    println!("{}", argument);
  }
}