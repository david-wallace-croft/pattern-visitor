use super::Element;
use std::cell::Cell;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct CircleElement {
  // Fields are public to provide access to the Visitor implementations
  pub center_x: Cell<f64>,
  pub center_y: Cell<f64>,
  id: usize,
  pub radius: f64,
}

impl CircleElement {
  pub fn new(id: usize) -> Self {
    Self {
      center_x: Cell::new(0.),
      center_y: Cell::new(0.),
      id,
      radius: 1.,
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
      "C({} {} {})",
      self.center_x.get(),
      self.center_y.get(),
      self.radius
    )
  }
}

impl Element for CircleElement {
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
