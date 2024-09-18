use super::Element;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct SquareElement {
  // Fields are public to provide access to the Visitor implementations
  pub center_x: f64,
  pub center_y: f64,
  pub half_height: f64,
}

impl SquareElement {
  pub fn new(half_height: f64) -> Self {
    Self {
      center_x: 0.,
      center_y: 0.,
      half_height,
    }
  }
}

impl Display for SquareElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "S({} {} {})",
      self.center_x, self.center_y, self.half_height
    )
  }
}

impl Element for SquareElement {
  fn get_center_x(&self) -> f64 {
    self.center_x
  }

  fn get_center_y(&self) -> f64 {
    self.center_y
  }

  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.center_x += offset_x;

    self.center_y += offset_y;
  }
}
