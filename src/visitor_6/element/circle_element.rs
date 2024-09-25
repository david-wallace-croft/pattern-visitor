use super::element_accessor::ElementAccessor;
use super::Element;
use ::std::cell::Cell;
use ::std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct CircleElement {
  // All fields are private; some use interior mutability
  center_x: Cell<f64>,
  center_y: Cell<f64>,
  id: usize,
  radius: Cell<f64>,
}

impl CircleElement {
  pub fn get_radius(&self) -> f64 {
    self.radius.get()
  }

  pub fn new(id: usize) -> Self {
    Self {
      center_x: Cell::new(0.),
      center_y: Cell::new(0.),
      id,
      radius: Cell::new(1.),
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

  pub fn set_radius(
    &self,
    radius: f64,
  ) {
    self.radius.set(radius);
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
      self.radius.get(),
    )
  }
}

impl Element for CircleElement {
  fn translate(
    &self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.set_center_x(self.center_x.get() + offset_x);

    self.set_center_y(self.center_y.get() + offset_y);
  }
}

impl ElementAccessor for CircleElement {
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
