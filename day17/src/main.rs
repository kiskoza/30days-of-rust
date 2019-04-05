#[macro_use] extern crate text_io;

mod calculator;
use calculator::Calculator;

fn main() {
  let n: i32 = read!();

  let calc = Calculator {};

  for _ in 0..n {
    let a: i32 = read!();
    let b: i32 = read!();

    match calc.power(a, b) {
      Ok(num) => println!("{}", num),
      Err(err) => println!("{}", err),
    }

  }
}
