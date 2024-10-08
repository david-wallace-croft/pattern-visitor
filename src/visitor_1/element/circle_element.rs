use super::Element;
use ::std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct CircleElement<T> {
  pub center_x: T,
  pub center_y: T,
  pub radius: T,
}

impl<T: Default> CircleElement<T> {
  pub fn new(radius: T) -> Self {
    Self {
      center_x: Default::default(),
      center_y: Default::default(),
      radius,
    }
  }
}

impl<T: Display> Display for CircleElement<T> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(f, "C({} {} {})", self.center_x, self.center_y, self.radius)
  }
}

impl Element for CircleElement<f64> {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.center_x += offset_x;

    self.center_y += offset_y;
  }
}

impl Element for CircleElement<isize> {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  ) {
    self.center_x = self
      .center_x
      .saturating_add(offset_x.round_ties_even() as isize);

    self.center_y = self
      .center_y
      .saturating_add(offset_y.round_ties_even() as isize);
  }
}
