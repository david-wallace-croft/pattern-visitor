use super::element::Element;
use super::value_element::ValueElement;
use super::visitor::Visitor;
use super::visitor_element::VisitorElement;

#[derive(Debug)]
pub struct GenericElement<T> {
  pub value: T,
}

impl Element for GenericElement<f64> {}
impl Element for GenericElement<String> {}
impl Element for GenericElement<usize> {}

impl ValueElement for GenericElement<f64> {
  fn double_value(&mut self) {
    self.value *= 2.;
  }
}

impl ValueElement for GenericElement<String> {
  fn double_value(&mut self) {
    self.value = self
      .value
      .repeat(2);
  }
}

impl ValueElement for GenericElement<usize> {
  fn double_value(&mut self) {
    self.value *= 2;
  }
}

impl VisitorElement for GenericElement<f64> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_generic_element_f64(self);
  }
}

impl VisitorElement for GenericElement<String> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_generic_element_string(self);
  }
}

impl VisitorElement for GenericElement<usize> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_generic_element_usize(self);
  }
}
