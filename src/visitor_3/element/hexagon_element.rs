use super::Element;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct HexagonElement {
  pub center_x: f64,
  pub center_y: f64,
  pub circumcircle_radius: f64,
  pub id: usize,
}

impl HexagonElement {
  pub fn new(
    id: usize,
    circumcircle_radius: f64,
  ) -> Self {
    Self {
      center_x: 0.,
      center_y: 0.,
      circumcircle_radius,
      id,
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
      "S[{}]({} {} {})",
      self.id, self.center_x, self.center_y, self.circumcircle_radius
    )
  }
}

impl Element for HexagonElement {
  fn get_center_x(&self) -> f64 {
    self.center_x
  }

  fn get_center_y(&self) -> f64 {
    self.center_y
  }

  fn get_circumcircle_radius(&self) -> f64 {
    self.circumcircle_radius
  }

  fn get_id(&self) -> usize {
    self.id
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
