mod item;
mod visitor;
mod half_value_visitor;
mod value_item;

use self::half_value_visitor::HalfValueVisitor;
use self::item::Item;
use self::value_item::ValueItem;

pub fn example() {
  let mut item_f64 = Item::<f64> { value: 1. };
  let mut item_string = Item::<String> { value: "1".to_string() };
  let mut item_usize = Item::<usize> { value: 1 };

  let mut items: Vec<&mut dyn ValueItem> = vec![
    &mut item_usize,
    &mut item_f64,
    &mut item_string,
  ];

  println!("{items:?}");

  items.iter_mut().for_each(|item| item.double_value());

  println!("{items:?}");

  items.iter_mut().for_each(|item| item.accept_visitor(&HalfValueVisitor));

  println!("{items:?}");
}
