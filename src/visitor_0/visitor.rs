use super::item::Item;

pub trait Visitor {
  fn visit_item_f64(&self, item: &mut Item<f64>);

  fn visit_item_string(&self, item: &mut Item<String>);

  fn visit_item_usize(&self, item: &mut Item<usize>);
}
