pub struct Calculator {

}

impl Calculator {
  pub fn power(&self, n: i32, p: i32) -> Result<i32, String> {
    if n < 0 || p < 0 {
      Err("Asd".to_string())
    } else {
      Ok(n.pow(p as u32))
    }
  }
}
