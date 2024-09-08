use super::visitor::Visitor;
use std::fmt::Debug;

pub trait Element: Debug {
  fn accept_visitor(&mut self, visitor: &dyn Visitor);
}
