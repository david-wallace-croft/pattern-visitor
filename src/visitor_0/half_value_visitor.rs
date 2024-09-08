use super::generic_element::GenericElement;
use super::visitor::Visitor;

pub struct HalfValueVisitor;

impl Visitor for HalfValueVisitor {
  fn visit_generic_element_f64(
    &self,
    element: &mut GenericElement<f64>,
  ) {
    element.value /= 2.;
  }

  fn visit_generic_element_string(
    &self,
    element: &mut GenericElement<String>,
  ) {
    let half_length: usize = element
      .value
      .len()
      / 2;

    element
      .value
      .truncate(half_length);
  }

  fn visit_generic_element_usize(
    &self,
    element: &mut GenericElement<usize>,
  ) {
    element.value /= 2;
  }
}
