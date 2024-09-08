use super::value_item::ValueItem;
use super::visitor::Visitor;

#[derive(Debug)]
pub struct Item<T> {
  pub value: T,
}

impl ValueItem for Item<f64> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_f64(self);
  }

  fn double_value(&mut self) {
    self.value *= 2.;
  }
}

impl ValueItem for Item<String> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_string(self);
  }

  fn double_value(&mut self) {
    self.value = self.value.repeat(2);
  }
}

impl ValueItem for Item<usize> {
  fn accept_visitor(&mut self, visitor: &dyn Visitor) {
    visitor.visit_item_usize(self);
  }

  fn double_value(&mut self) {
    self.value *= 2;
  }
}
