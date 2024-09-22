pub mod element;
pub mod visitor;

use self::element::circle_element::CircleElement;
use self::element::point_element::PointElement;
use self::element::square_element::SquareElement;
use self::visitor::scale_visitor::ScaleVisitor;
use self::visitor::visitor_element::VisitorElement;

pub fn example_original() {
  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(PointElement::default()),
    Box::new(SquareElement::new(1.)),
  ];

  println!("=== visitor_2 original ===");

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
