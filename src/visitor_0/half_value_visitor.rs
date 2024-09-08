use super::item::Item;
use super::visitor::Visitor;

pub struct HalfValueVisitor;

impl Visitor for HalfValueVisitor {
  fn visit_item(&self, item: &mut Item) {
    item.value /= 2;
  }
}
