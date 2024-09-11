pub mod element;
pub mod visitor;

use self::element::hexagon_element::HexagonElement;
use self::visitor::visitor_element::VisitorElement;
use super::original::element::circle_element::CircleElement;
use super::original::element::square_element::SquareElement;
use super::original::visitor::scale_visitor::ScaleVisitor;

pub fn example_supplemented() {
  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(SquareElement::new(1.)),
    Box::new(HexagonElement::new(1.)),
  ];

  println(&visitor_elements);

  visitor_elements
    .iter_mut()
    .for_each(|element| element.translate(1., 0.));

  println(&visitor_elements);

  let scale_visitor = ScaleVisitor::new(2.);

  visitor_elements
    .iter_mut()
    .for_each(|visitor_element| visitor_element.accept_visitor(&scale_visitor));

  println(&visitor_elements);
}

fn println(visitor_elements: &[Box<dyn VisitorElement>]) {
  visitor_elements
    .iter()
    .for_each(|visitor_element| print!("{visitor_element} "));

  println!();
}
