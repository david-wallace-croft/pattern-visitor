use super::item::Item;
use super::visitor::Visitor;

pub struct HalfValueVisitor;

impl Visitor for HalfValueVisitor {
  fn visit_item_f64(&self, item: &mut Item<f64>) {
    item.value /= 2.;
  }

  fn visit_item_string(&self, item: &mut Item<String>) {
    let half_length: usize = item.value.len() / 2;

    item.value.truncate(half_length);
  }

  fn visit_item_usize(&self, item: &mut Item<usize>) {
    item.value /= 2;
  }
}
