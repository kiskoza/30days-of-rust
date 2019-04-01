#[macro_use] extern crate text_io;

mod person;
use person::Student;
use person::Person;

fn main() {
  let first_name: String = read!("{}");
  let last_name: String = read!("{}");
  let id: i32 = read!();

  let number_of_scores: i32 = read!();
  let mut scores = vec!();

  for _ in 0..number_of_scores {
    let score: i32 = read!();
    scores.push(score);
  }

  let student = Student::new(first_name, last_name, id, &scores);
  student.print_person();
  println!("Grade: {}", student.calculate());
}
