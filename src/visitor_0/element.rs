use super::visitor::Visitor;
use std::fmt::{Debug, Display};

pub trait Element: Debug + Display {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  );

  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  );
}
