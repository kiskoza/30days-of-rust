#[macro_use] extern crate text_io;

mod book;
use book::Book;

mod my_book;
use my_book::MyBook;

fn main() {
  let title: String = read!("{}\n");
  let author: String = read!("{}\n");
  let price: i32 = read!();

  let book = MyBook { title, author, price };
  book.display();
}
