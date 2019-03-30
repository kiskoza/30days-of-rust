#[macro_use] extern crate text_io;

fn factorial(n: i32) -> i32 {
  match n {
    0...1 => n,
    _ => factorial(n - 1) * n,
  }
}

fn main() {
  let n: i32 = read!();

  println!("{}", factorial(n));
}
