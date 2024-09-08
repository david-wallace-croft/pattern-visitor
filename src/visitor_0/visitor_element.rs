use super::visitor::Visitor;

pub trait VisitorElement {
  fn accept_visitor(&mut self, visitor: &dyn Visitor);
}
