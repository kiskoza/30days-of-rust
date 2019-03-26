#[macro_use] extern crate text_io;

fn main() {
  let t: i32 = read!();

  for _ in 0..t {
    let str: String = read!();
    let even: String = str.chars().enumerate().filter(|&(index, _)| index % 2 == 0).map(|(_, char)| char).collect();
    let odd: String = str.chars().enumerate().filter(|&(index, _)| index % 2 == 1).map(|(_, char)| char).collect();
    println!("{} {}", even, odd);
  };
}
