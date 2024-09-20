use super::Element;
use std::cell::Cell;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct SquareElement {
  // Fields are public to provide access to the Visitor implementations
  pub center_x: Cell<f64>,
  pub center_y: Cell<f64>,
  pub half_height: f64,
  id: usize,
}

impl SquareElement {
  pub fn new(id: usize) -> Self {
    Self {
      center_x: Cell::new(0.),
      center_y: Cell::new(0.),
      half_height: 1.,
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
      "S({} {} {})",
      self.center_x.get(),
      self.center_y.get(),
      self.half_height
    )
  }
}

impl Element for SquareElement {
  fn get_center_x(&self) -> f64 {
    self.center_x.get()
  }

  fn get_center_y(&self) -> f64 {
    self.center_y.get()
  }

  fn get_id(&self) -> usize {
    self.id
  }

  fn set_center_x(
    &self,
    center_x: f64,
  ) {
    self.center_x.set(center_x);
  }

  fn set_center_y(
    &self,
    center_y: f64,
  ) {
    self.center_y.set(center_y);
  }

  fn translate(
    &self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.set_center_x(self.get_center_x() + offset_x);

    self.set_center_y(self.get_center_y() + offset_y);
  }
}
