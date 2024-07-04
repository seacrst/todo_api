pub struct Path {
  pub base: String
}

impl Path {
  pub fn define(&self, path: &String) -> String {
    self.base.to_owned() + path
  }
}