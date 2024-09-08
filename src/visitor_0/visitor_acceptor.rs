use super::visitor::Visitor;

pub trait VisitorAcceptor {
  fn accept_visitor(&mut self, visitor: &dyn Visitor);
}
