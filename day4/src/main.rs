#[macro_use] extern crate text_io;

mod person;
use person::Person;

fn main() {
  let t: i32 = read!();

  for _ in 0..t {
    let age: i32 = read!();
    let mut p = Person::new(age);

    p.am_i_old();

    for _ in 0..3 {
      p.year_passess();
    }

    p.am_i_old();
    println!("");
  }
}
