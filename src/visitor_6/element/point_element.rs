use super::Element;
use ::std::cell::Cell;
use ::std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Default)]
pub struct PointElement {
  // All fields are private
  x: Cell<f64>,
  y: Cell<f64>,
  id: usize,
}

impl PointElement {
  pub fn new(id: usize) -> Self {
    Self {
      x: Cell::new(0.),
      y: Cell::new(0.),
      id,
    }
  }
}

impl Display for PointElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "P({} {})", self.x.get(), self.y.get())
  }
}

impl Element for PointElement {
  fn get_center_x(&self) -> f64 {
    self.x.get()
  }

  fn get_center_y(&self) -> f64 {
    self.y.get()
  }

  fn get_id(&self) -> usize {
    self.id
  }

  fn set_center_x(
    &self,
    center_x: f64,
  ) {
    self.x.set(center_x);
  }

  fn set_center_y(
    &self,
    center_y: f64,
  ) {
    self.y.set(center_y);
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
