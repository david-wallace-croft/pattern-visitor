use super::value_element::ValueElement;
use super::visitor::Visitor;
use super::visitor_element::VisitorElement;

#[derive(Debug)]
pub struct Item<T> {
  pub value: T,
}

impl ValueElement for Item<f64> {
  fn double_value(&mut self) {
    self.value *= 2.;
  }
}

impl ValueElement for Item<String> {
  fn double_value(&mut self) {
    self.value = self.value.repeat(2);
  }
}

impl ValueElement for Item<usize> {
  fn double_value(&mut self) {
    self.value *= 2;
  }
}

impl VisitorElement for Item<f64> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_f64(self);
  }
}

impl VisitorElement for Item<String> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_string(self);
  }
}

impl VisitorElement for Item<usize> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_usize(self);
  }
}
