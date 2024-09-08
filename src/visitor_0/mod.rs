mod item;
mod visitor;
mod half_value_visitor;
mod value_element;
mod visitor_element;

use self::half_value_visitor::HalfValueVisitor;
use self::item::Item;
use self::value_element::ValueElement;
use self::visitor_element::VisitorElement;
use std::fmt::Debug;

pub fn example() {
  let mut item_0 = Item::<usize> { value: 1 };
  let mut item_1 = Item::<f64> { value: 1. };
  let mut item_2 = Item::<String> { value: "1".to_string() };

  let debugs: Vec<&dyn Debug> = vec![&item_0, &item_1, &item_2];

  println!("{debugs:?}");

  let mut value_elements: Vec<&mut dyn ValueElement> = vec![
    &mut item_0,
    &mut item_1,
    &mut item_2,
  ];

  value_elements.iter_mut().for_each(|value_element|
    value_element.double_value());

  let debugs: Vec<&dyn Debug> = vec![&item_0, &item_1, &item_2];

  println!("{debugs:?}");

  let mut visitor_elements: Vec<&mut dyn VisitorElement> = vec![
    &mut item_0,
    &mut item_1,
    &mut item_2,
  ];

  visitor_elements.iter_mut().for_each(|visitor_acceptor|
    visitor_acceptor.accept_visitor(&HalfValueVisitor));

  let debugs: Vec<&dyn Debug> = vec![&item_0, &item_1, &item_2];

  println!("{debugs:?}");
}
