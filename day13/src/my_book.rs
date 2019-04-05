use crate::book::Book;

pub struct MyBook {
  pub title: String,
  pub author: String,
  pub price: i32,
}

impl Book for MyBook {
  fn display(&self) {
    println!("Title: {}", self.title);
    println!("Author: {}", self.author);
    println!("Price: {}", self.price);
  }
}
