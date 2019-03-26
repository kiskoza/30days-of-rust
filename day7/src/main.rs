#[macro_use] extern crate text_io;

fn main() {
  let n: i32 = read!();
  let mut vec = Vec::new();

  for _ in 0..n {
    let i: i32 = read!("{}");
    vec.push(i);
  }

  vec.reverse();
  vec.iter().for_each( |i| print!("{} ", i));
}
