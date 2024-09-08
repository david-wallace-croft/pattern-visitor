mod circle_element;
mod element;
mod scale_visitor;
mod square_element;
mod visitor;

use self::circle_element::CircleElement;
use self::element::Element;
use self::scale_visitor::ScaleVisitor;
use self::square_element::SquareElement;

pub fn example() {
  let mut elements: Vec<Box<dyn Element>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(SquareElement::new(1.)),
  ];

  println(&elements);

  elements
    .iter_mut()
    .for_each(|element| element.translate(1., 0.));

  println(&elements);

  let scale_visitor = ScaleVisitor::new(2.);

  elements
    .iter_mut()
    .for_each(|element| element.accept_visitor(&scale_visitor));

  println(&elements);
}

fn println(elements: &Vec<Box<dyn Element>>) {
  elements
    .iter()
    .for_each(|element| print!("{element} "));

  println!();
}
