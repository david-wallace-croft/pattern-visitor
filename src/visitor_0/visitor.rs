use super::generic_element::GenericElement;

pub trait Visitor {
  fn visit_item_f64(
    &self,
    item: &mut GenericElement<f64>,
  );

  fn visit_item_string(
    &self,
    item: &mut GenericElement<String>,
  );

  fn visit_item_usize(
    &self,
    item: &mut GenericElement<usize>,
  );
}
