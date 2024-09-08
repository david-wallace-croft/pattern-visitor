use super::visitor::Visitor;

pub trait VisitorAcceptor {
  fn accept_visitor(&self, visitor: &Visitor);
}
