use super::super::super::original::element::Element;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct HexagonElement {
  pub center_x: f64,
  pub center_y: f64,
  pub circumcircle_radius: f64,
}

impl HexagonElement {
  pub fn new(circumcircle_radius: f64) -> Self {
    Self {
      center_x: 0.,
      center_y: 0.,
      circumcircle_radius,
    }
  }
}

impl Display for HexagonElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "H({} {} {})",
      self.center_x, self.center_y, self.circumcircle_radius
    )
  }
}

impl Element for HexagonElement {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.center_x += offset_x;

    self.center_y += offset_y;
  }
}
