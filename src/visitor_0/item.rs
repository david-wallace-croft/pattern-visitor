use super::value_item::ValueItem;
use super::visitor::Visitor;
use super::visitor_acceptor::VisitorAcceptor;

#[derive(Debug)]
pub struct Item<T> {
  pub value: T,
}

impl ValueItem for Item<f64> {
  fn double_value(&mut self) {
    self.value *= 2.;
  }
}

impl ValueItem for Item<String> {
  fn double_value(&mut self) {
    self.value = self.value.repeat(2);
  }
}

impl ValueItem for Item<usize> {
  fn double_value(&mut self) {
    self.value *= 2;
  }
}

impl VisitorAcceptor for Item<f64> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_f64(self);
  }
}

impl VisitorAcceptor for Item<String> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_string(self);
  }
}

impl VisitorAcceptor for Item<usize> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_usize(self);
  }
}
