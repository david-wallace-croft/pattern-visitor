use super::item::Item;

pub trait Visitor<T> {
  fn visit(&self, item: &mut Item<T>);
}
