use super::visitor::Visitor;

pub trait VisitorAcceptor<T> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor<T>);
}
