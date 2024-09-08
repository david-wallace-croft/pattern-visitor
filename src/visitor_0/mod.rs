mod item;
mod visitor;
mod half_value_visitor;
mod value_item;
mod visitor_acceptor;

use self::half_value_visitor::HalfValueVisitor;
use self::item::Item;
use self::value_item::ValueItem;
use self::visitor_acceptor::VisitorAcceptor;
use std::fmt::Debug;

pub fn example() {
  let mut item_0 = Item::<usize> { value: 1 };
  let mut item_1 = Item::<f64> { value: 1. };
  let mut item_2 = Item::<String> { value: "1".to_string() };

  let debugs: Vec<&dyn Debug> = vec![&item_0, &item_1, &item_2];

  println!("{debugs:?}");

  let mut value_items: Vec<&mut dyn ValueItem> = vec![
    &mut item_0,
    &mut item_1,
    &mut item_2,
  ];

  value_items.iter_mut().for_each(|value_item| value_item.double_value());

  let debugs: Vec<&dyn Debug> = vec![&item_0, &item_1, &item_2];

  println!("{debugs:?}");

  let mut visitor_acceptors: Vec<&mut dyn VisitorAcceptor> = vec![
    &mut item_0,
    &mut item_1,
    &mut item_2,
  ];

  visitor_acceptors.iter_mut().for_each(|visitor_acceptor|
    visitor_acceptor.accept_visitor(&HalfValueVisitor));

  let debugs: Vec<&dyn Debug> = vec![&item_0, &item_1, &item_2];

  println!("{debugs:?}");
}
