use super::visitor::Visitor;
use std::fmt::Debug;

pub trait ValueItem: Debug {
  fn accept_visitor(&mut self, visitor: &dyn Visitor);

  fn double_value(&mut self);
}
