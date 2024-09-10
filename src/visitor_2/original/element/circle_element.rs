use super::Element;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct CircleElement {
  pub center_x: f64,
  pub center_y: f64,
  pub radius: f64,
}

impl CircleElement {
  pub fn new(radius: f64) -> Self {
    Self {
      center_x: 0.,
      center_y: 0.,
      radius,
    }
  }
}

impl Display for CircleElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "C({} {} {})", self.center_x, self.center_y, self.radius)
  }
}

impl Element for CircleElement {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.center_x += offset_x;

    self.center_y += offset_y;
  }
}
