#[macro_use] extern crate text_io;

fn main() {
  let n: i32 = read!();

  for i in 1..11 {
    println!("{} x {} = {}", n, i, n * i);
  }
}
