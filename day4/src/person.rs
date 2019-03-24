pub struct Person {
  age: i32
}

impl Person {
  pub fn am_i_old(&self) {
     if self.age < 13 {
       println!("You are young.");
     } else if self.age < 18 {
       println!("You are a teenager.")
     } else {
       println!("You are old.");
     }
   }

  pub fn new(age: i32) -> Person {
    let age = if age < 0 {
                println!("Age is not valid, setting age to 0.");
                0
              } else {
                age
              };
    Person { age }
  }

  pub fn year_passess(&mut self) {
    self.age = self.age + 1;
  }
}
