use super::visitor::Visitor;

#[derive(Debug)]
pub struct Item<T> {
  pub value: T,
}

impl<T> Item<T> {
  pub fn accept_visitor(&mut self, visitor: &dyn Visitor<T>) {
    visitor.visit(self)
  }
}

impl Item<f64> {
  pub fn double_value(&mut self) {
    self.value *= 2.;
  }
}

impl Item<usize> {
  pub fn double_value(&mut self) {
    self.value *= 2;
  }
}
