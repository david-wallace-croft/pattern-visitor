use self::element::circle_element::CircleElement;
use self::element::square_element::SquareElement;
use self::element::Element;
use self::visitor::scale_visitor::ScaleVisitor;

mod element;
mod visitor;

pub fn example() {
  let mut elements: Vec<Box<dyn Element>> = vec![
    Box::new(CircleElement::<f64>::new(1.)),
    Box::new(SquareElement::<f64>::new(1.)),
    Box::new(CircleElement::<isize>::new(1)),
    Box::new(SquareElement::<isize>::new(1)),
  ];

  println(&elements);

  elements
    .iter_mut()
    .for_each(|element| element.translate(1.5, 0.));

  println(&elements);

  let scale_visitor = ScaleVisitor::new(1.5);

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
