#[macro_use] extern crate text_io;

fn main() {
  let i = 4;
  let d = 4.0;
  let s = "HackerRank ";

  // read the values
  let i2: i32 = read!();
  let d2: f32 = read!();
  let s2: String = read!("{}\n");

  println!("{}", i + i2);
  println!("{:.1}", d + d2);
  println!("{}{}", s, s2);
}
