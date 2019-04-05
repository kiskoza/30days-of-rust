#[macro_use] extern crate text_io;

mod difference;
use difference::Difference;

fn main() {
  let n: i32 = read!();
  let mut array = vec!();

  for _ in 0..n {
    let i: i32 = read!();
    array.push(i);
  }

  let d = Difference { array: &array };

  println!("{}", d.compute_maximum_difference());
}
