use super::Element;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct CircleElement {
  pub center_x: f64,
  pub center_y: f64,
  pub id: usize,
  pub radius: f64,
}

impl CircleElement {
  pub fn new(
    id: usize,
    radius: f64,
  ) -> Self {
    Self {
      center_x: 0.,
      center_y: 0.,
      id,
      radius,
    }
  }
}

impl Display for CircleElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(
      f,
      "C[{}]({} {} {})",
      self.id, self.center_x, self.center_y, self.radius
    )
  }
}

impl Element for CircleElement {
  fn get_center_x(&self) -> f64 {
    self.center_x
  }

  fn get_center_y(&self) -> f64 {
    self.center_y
  }

  fn get_circumcircle_radius(&self) -> f64 {
    self.radius
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
