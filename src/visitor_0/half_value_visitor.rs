use super::generic_element::GenericElement;
use super::visitor::Visitor;

pub struct HalfValueVisitor;

impl Visitor for HalfValueVisitor {
  fn visit_item_f64(
    &self,
    generic_element: &mut GenericElement<f64>,
  ) {
    generic_element.value /= 2.;
  }

  fn visit_item_string(
    &self,
    generic_element: &mut GenericElement<String>,
  ) {
    let half_length: usize = generic_element
      .value
      .len()
      / 2;

    generic_element
      .value
      .truncate(half_length);
  }

  fn visit_item_usize(
    &self,
    generic_element: &mut GenericElement<usize>,
  ) {
    generic_element.value /= 2;
  }
}
