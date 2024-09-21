use self::element::circle_element::CircleElement;
use self::element::point_element::PointElement;
use self::element::square_element::SquareElement;
use self::visitor::scale_visitor::ScaleVisitor;
use self::visitor::visitor_element::VisitorElement;

mod element;
mod visitor;

pub fn example() {
  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::<f64>::new(1.)),
    Box::new(PointElement::<f64>::default()),
    Box::new(SquareElement::<f64>::new(1.)),
    Box::new(CircleElement::<isize>::new(1)),
    Box::new(PointElement::<isize>::default()),
    Box::new(SquareElement::<isize>::new(1)),
  ];

  println!("=== visitor_1 ===");

  println(&visitor_elements);

  visitor_elements
    .iter_mut()
    .for_each(|visitor_element| visitor_element.translate(1.5, 0.));

  println(&visitor_elements);

  let scale_visitor = ScaleVisitor::new(1.5);

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
