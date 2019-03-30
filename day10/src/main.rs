#[macro_use] extern crate text_io;

use itertools::Itertools;

fn main() {
  let n: i32 = read!();

  let mut bits = vec!();

  let mut i = n;
  while i > 0 {
    bits.push(i % 2);
    i = i / 2;
  }

  let max = bits
    .into_iter()
    .map(|c| (c, 1))
    .coalesce( |(x, nx), (y, ny)|
      if x == y {
        Ok((x, nx + ny))
      } else {
        Err(((x, nx), (y, ny)))
      }
    )
    .filter(|(c, _)| *c == 1)
    .map(|(_, n)| n)
    .max()
    .unwrap();

  println!("{}", max)
}
