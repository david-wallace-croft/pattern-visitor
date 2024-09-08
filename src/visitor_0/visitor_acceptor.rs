use super::visitor::Visitor;
use std::fmt::Debug;

pub trait VisitorAcceptor: Debug {
  fn accept_visitor(&mut self, visitor: &dyn Visitor);
}
