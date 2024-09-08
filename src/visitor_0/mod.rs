mod element;
mod generic_element;
mod half_value_visitor;
mod value_element;
mod visitor;
mod visitor_element;

use self::element::Element;
use self::generic_element::GenericElement;
use self::half_value_visitor::HalfValueVisitor;

pub fn example() {
  let mut item_0 = GenericElement::<usize> {
    value: 1,
  };

  let mut item_1 = GenericElement::<f64> {
    value: 1.,
  };

  let mut item_2 = GenericElement::<String> {
    value: "1".to_string(),
  };

  let mut elements: Vec<&mut dyn Element> = vec![
    &mut item_0,
    &mut item_1,
    &mut item_2,
  ];

  println!("{elements:?}");

  elements
    .iter_mut()
    .for_each(|element| element.double_value());

  println!("{elements:?}");

  elements
    .iter_mut()
    .for_each(|element| element.accept_visitor(&HalfValueVisitor));

  println!("{elements:?}");
}
