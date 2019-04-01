use crate::person::Person;

pub struct Student<'a> {
  first_name: String,
  last_name: String,
  id: i32,
  scores: &'a[i32]
}

impl<'a> Person for Student<'a> {
  fn new(first_name: String, last_name: String, id: i32) -> Student<'a> {
    Student { first_name, last_name, id, scores: &[] }
  }

  fn last_name(&self) -> &String {
    Student::last_name(self)
  }

  fn first_name(&self) -> &String {
    Student::first_name(self)
  }

  fn id(&self) -> &i32 {
    Student::id(self)
  }
}

impl<'a> Student<'a> {
  pub fn new(first_name: String, last_name: String, id: i32, scores: &'a [i32]) -> Student<'a> {
    let mut s: Student<'a> = Person::new(first_name, last_name, id);
    s.scores = scores;
    s
  }

  pub fn calculate(&self) -> String {
    let average = self.scores.iter().sum::<i32>() as f32 / self.scores.len() as f32;
    if average < 40.0 {
      String::from("T")
    } else if average < 55.0 {
      String::from("D")
    } else if average < 70.0 {
      String::from("P")
    } else if average < 80.0 {
      String::from("A")
    } else if average < 90.0 {
      String::from("E")
    } else {
      String::from("O")
    }
  }

  pub fn last_name(&self) -> &String {
    &self.last_name
  }

  pub fn first_name(&self) -> &String {
    &self.first_name
  }

  pub fn id(&self) -> &i32 {
    &self.id
  }
}
