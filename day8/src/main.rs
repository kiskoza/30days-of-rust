#[macro_use] extern crate text_io;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

fn main() {
  let n: i32 = read!();

  let mut book = HashMap::new();

  for _ in 0..n {
    let name: String = read!("{}");
    let number: i32 = read!();

    book.insert(name, number);
  }

  let regex = Regex::new(r"^[a-z]+$").unwrap();

  loop {
    let name: String = read!("{}\n");

    if !regex.is_match(&name) {
      return;
    }

    match book.get(&name) {
      Some(&number) => println!("{}={}", name, number),
      _ => println!("Not found"),
    };
  }
}
