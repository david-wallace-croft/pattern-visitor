use super::generic_element::GenericElement;

pub trait Visitor {
  fn visit_generic_element_f64(
    &self,
    element: &mut GenericElement<f64>,
  );

  fn visit_generic_element_string(
    &self,
    element: &mut GenericElement<String>,
  );

  fn visit_generic_element_usize(
    &self,
    element: &mut GenericElement<usize>,
  );
}
