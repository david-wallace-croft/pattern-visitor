use super::visitor::Visitor;
use super::visitor_acceptor::VisitorAcceptor;

#[derive(Debug)]
pub struct Item {
  pub health: isize,
  pub wealth: f64,
  pub wisdom: usize,
}

impl VisitorAcceptor for Item {
  fn accept_visitor(&self, visitor: &Visitor) {
    visitor.visit_item(self)
  }
}
