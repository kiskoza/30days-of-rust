#[macro_use] extern crate text_io;

fn main() {
  let i: Result<i32, _> = try_read!();

  match i {
    Ok(num) => println!("{}", num),
    Err(_) => println!("Bad String"),
  }
}
