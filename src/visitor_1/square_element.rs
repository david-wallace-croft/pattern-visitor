use super::element::Element;
use super::visitor::Visitor;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct SquareElement<T> {
  pub center_x: T,
  pub center_y: T,
  pub half_height: T,
}

impl<T: Default> SquareElement<T> {
  pub fn new(half_height: T) -> Self {
    Self {
      center_x: Default::default(),
      center_y: Default::default(),
      half_height,
    }
  }
}

impl<T: Display> Display for SquareElement<T> {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    write!(
      f,
      "S({} {} {})",
      self.center_x, self.center_y, self.half_height
    )
  }
}

impl Element for SquareElement<f64> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_square_element_f64(self);
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

impl Element for SquareElement<isize> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_square_element_isize(self);
  }

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
