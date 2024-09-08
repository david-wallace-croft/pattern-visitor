use super::item::Item;

pub trait Visitor {
  fn visit_item(&self, item: &mut Item);
}
