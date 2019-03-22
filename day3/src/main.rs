#[macro_use] extern crate text_io;

fn main() {
  let n: i32 = read!();

  // Only in nightly: (6..20).contains(&n)
  if n % 2 == 1 || (6 <= n && n <= 20) {
    println!("Weird");
  } else {
    println!("Not Weird");
  }
}
