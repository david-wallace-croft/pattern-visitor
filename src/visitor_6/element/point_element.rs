use super::element_accessor::ElementAccessor;
use super::Element;
use ::std::cell::Cell;
use ::std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct PointElement {
  // All fields are private; some use interior mutability
  id: usize,
  x: Cell<f64>,
  y: Cell<f64>,
}

impl PointElement {
  pub fn new(id: usize) -> Self {
    Self {
      id,
      x: Cell::new(0.),
      y: Cell::new(0.),
    }
  }

  pub fn set_x(
    &self,
    x: f64,
  ) {
    self.x.set(x);
  }

  pub fn set_y(
    &self,
    y: f64,
  ) {
    self.y.set(y);
  }
}

impl Display for PointElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "P({} {})", self.x.get(), self.y.get())
  }
}

impl Element for PointElement {
  fn translate(
    &self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.set_x(self.x.get() + offset_x);

    self.set_y(self.y.get() + offset_y);
  }
}

impl ElementAccessor for PointElement {
  fn get_center_x(&self) -> f64 {
    self.x.get()
  }

  fn get_center_y(&self) -> f64 {
    self.y.get()
  }

  fn get_id(&self) -> usize {
    self.id
  }
}
