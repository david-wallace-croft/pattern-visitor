use super::Element;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct PointElement {
  // Fields are public to provide access to the Visitor implementations
  pub center_x: f64,
  pub center_y: f64,
}

impl Display for PointElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "P({} {})", self.center_x, self.center_y)
  }
}

impl Element for PointElement {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.center_x += offset_x;

    self.center_y += offset_y;
  }
}
