pub trait Person {
  fn new(first_name: String, last_name: String, id: i32) -> Self;
  fn print_person(&self) {
    println!("Name: {}, {}", self.last_name(), self.first_name());
    println!("ID: {}", self.id());
  }

  fn last_name(&self) -> &String;
  fn first_name(&self) -> &String;
  fn id(&self) -> &i32;
}
