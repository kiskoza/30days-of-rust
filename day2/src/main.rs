#[macro_use] extern crate text_io;

fn solve(meal_cost: f32, tip_percent: i32, tax_percent: i32) -> i32 {
  (meal_cost * (100 + tip_percent + tax_percent) as f32 / 100.0).round() as i32
}

fn main() {
  let meal_cost: f32 = read!();
  let tip_percent: i32 = read!();
  let tax_percent: i32 = read!();

  println!("{}", solve(meal_cost, tip_percent, tax_percent));
}
