use super::visitor::Visitor;
use super::visitor_acceptor::VisitorAcceptor;

#[derive(Debug)]
pub struct Item {
  pub value: usize,
}

impl Item {
  pub fn double_value(&mut self) {
    self.value *= 2;
  }
}

impl VisitorAcceptor for Item {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item(self)
  }
}
