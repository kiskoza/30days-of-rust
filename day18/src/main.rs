#[macro_use] extern crate text_io;

fn main() {
  let word: String = read!();

  let forward = word.chars();
  let backward = word.chars().rev();

  if forward.zip(backward).all(|(f, b)| f == b) {
    println!("The word, {}, is a palindrome.", word);
  } else {
    println!("The word, {}, is not a palindrome.", word);
  }
}
