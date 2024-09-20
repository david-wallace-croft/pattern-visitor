//==============================================================================
//! Demonstration of the "Visitor" pattern.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-18
//! - Updated: 2024-09-19
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

mod element;
mod visitor;

use self::element::circle_element::CircleElement;
use self::element::point_element::PointElement;
use self::element::square_element::SquareElement;
use self::visitor::cluster_visitor::ClusterVisitor;
use self::visitor::visitor_element::VisitorElement;
use std::ops::IndexMut;

pub fn example() {
  println!("=== visitor_5 ===");

  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(PointElement::default()),
    Box::new(SquareElement::new(1.)),
  ];

  println(&visitor_elements);

  visitor_elements
    .iter_mut()
    .enumerate()
    .for_each(|(index, element)| element.translate((index + 1) as f64, 0.));

  println(&visitor_elements);

  let length = visitor_elements.len();

  for index in 0..length {
    let cluster_visitor = ClusterVisitor::new(index, &visitor_elements);

    let visitor_element: &mut Box<dyn VisitorElement> =
      visitor_elements.index_mut(index);

    visitor_element.accept_visitor(&cluster_visitor);
  }

  println(&visitor_elements);
}

fn println(visitor_elements: &[Box<dyn VisitorElement>]) {
  visitor_elements
    .iter()
    .for_each(|visitor_element| print!("{visitor_element} "));

  println!();
}
