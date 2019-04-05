#[macro_use] extern crate text_io;

fn main() {
  let n: i32 = read!();

  let mut vec = vec!();

  for _ in 0..n {
    let i: i32 = read!();
    vec.push(i);
  }

  for i in vec {
    print!("{} ", i);
  }
}
