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
  let mut elements: Vec<Box<dyn Element>> = vec![
    Box::new(GenericElement::<usize> {
      value: 1,
    }),
    Box::new(GenericElement::<f64> {
      value: 1.,
    }),
    Box::new(GenericElement::<String> {
      value: "1".to_string(),
    }),
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
