mod item;
mod visitor;
mod half_value_visitor;
mod value_item;
mod visitor_acceptor;

use self::half_value_visitor::HalfValueVisitor;
use self::item::Item;
use self::value_item::ValueItem;
use self::visitor_acceptor::VisitorAcceptor;

pub fn example() {
  let mut item_f64 = Item::<f64> { value: 1. };
  let mut item_string = Item::<String> { value: "1".to_string() };
  let mut item_usize = Item::<usize> { value: 1 };

  let mut value_items: Vec<&mut dyn ValueItem> = vec![
    &mut item_usize,
    &mut item_f64,
    &mut item_string,
  ];

  println!("{value_items:?}");

  value_items.iter_mut().for_each(|value_item| value_item.double_value());

  println!("{value_items:?}");

  let mut visitor_acceptors: Vec<&mut dyn VisitorAcceptor> = vec![
    &mut item_usize,
    &mut item_f64,
    &mut item_string,
  ];

  visitor_acceptors.iter_mut().for_each(|visitor_acceptor|
    visitor_acceptor.accept_visitor(&HalfValueVisitor));

  println!("{visitor_acceptors:?}");
}
