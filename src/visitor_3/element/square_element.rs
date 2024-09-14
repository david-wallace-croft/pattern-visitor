use super::Element;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct SquareElement {
  pub center_x: f64,
  pub center_y: f64,
  pub half_diagonal: f64,
  pub id: usize,
}

impl SquareElement {
  pub fn new(
    id: usize,
    half_diagonal: f64,
  ) -> Self {
    Self {
      center_x: 0.,
      center_y: 0.,
      half_diagonal,
      id,
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
      "S[{}]({} {} {})",
      self.id, self.center_x, self.center_y, self.half_diagonal
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

  fn get_circumcircle_radius(&self) -> f64 {
    self.half_diagonal
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
