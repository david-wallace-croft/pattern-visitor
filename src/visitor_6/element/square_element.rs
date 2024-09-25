use super::element_accessor::ElementAccessor;
use super::Element;
use ::std::cell::Cell;
use ::std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct SquareElement {
  // All fields are private; some use interior mutability
  center_x: Cell<f64>,
  center_y: Cell<f64>,
  half_diagonal: Cell<f64>,
  id: usize,
}

impl SquareElement {
  pub fn get_half_diagonal(&self) -> f64 {
    self.half_diagonal.get()
  }

  pub fn new(id: usize) -> Self {
    Self {
      center_x: Cell::new(0.),
      center_y: Cell::new(0.),
      half_diagonal: Cell::new(1.),
      id,
    }
  }

  pub fn set_center_x(
    &self,
    center_x: f64,
  ) {
    self.center_x.set(center_x);
  }

  pub fn set_center_y(
    &self,
    center_y: f64,
  ) {
    self.center_y.set(center_y);
  }

  pub fn set_half_diagonal(
    &self,
    half_diagonal: f64,
  ) {
    self.half_diagonal.set(half_diagonal);
  }
}

impl Display for SquareElement {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(
      f,
      "S({} {} {})",
      self.center_x.get(),
      self.center_y.get(),
      self.half_diagonal.get(),
    )
  }
}

impl Element for SquareElement {
  fn translate(
    &self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.set_center_x(self.center_x.get() + offset_x);

    self.set_center_y(self.center_y.get() + offset_y);
  }
}

impl ElementAccessor for SquareElement {
  fn get_center_x(&self) -> f64 {
    self.center_x.get()
  }

  fn get_center_y(&self) -> f64 {
    self.center_y.get()
  }

  fn get_id(&self) -> usize {
    self.id
  }
}
