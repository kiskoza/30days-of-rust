use itertools::Itertools;

pub struct Difference<'a> {
  pub array: &'a[i32],
}

impl<'a> Difference<'a> {
  pub fn compute_maximum_difference(&self) -> i32 {
    match self.array.into_iter().tuple_combinations().map(|(a, b)| (a - b).abs() ).max() {
      Some(num) => num,
      None => 0
    }
  }
}
