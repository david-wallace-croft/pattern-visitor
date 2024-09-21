use super::Element;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct PointElement<T> {
  // Fields are public to provide access to the Visitor implementations
  pub x: T,
  pub y: T,
}

impl<T: Default> Default for PointElement<T> {
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
    }
  }
}

impl<T: Display> Display for PointElement<T> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "P({} {})", self.x, self.y)
  }
}

impl Element for PointElement<f64> {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.x += offset_x;

    self.y += offset_y;
  }
}

impl Element for PointElement<isize> {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.x = self.x.saturating_add(offset_x.round_ties_even() as isize);

    self.y = self.y.saturating_add(offset_y.round_ties_even() as isize);
  }
}
