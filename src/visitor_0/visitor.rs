use super::item::Item;

pub struct Visitor {}

impl Visitor {
  pub fn visit_item(&self, item: &Item) {
    println!("item: {item:?}");
  }
}
