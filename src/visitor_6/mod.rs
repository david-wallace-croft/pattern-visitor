//==============================================================================
//! Demonstration of the "Visitor" pattern.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-19
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
use self::visitor::visitor_element::VisitorElement;
use crate::visitor_6::visitor::cluster_visitor::ClusterVisitor;
use std::cell::RefCell;
use std::rc::Rc;

pub fn example() {
  println!("=== visitor_6 ===");

  let visitor_elements: Rc<RefCell<Vec<Box<dyn VisitorElement>>>> =
    Rc::new(RefCell::new(vec![
      Box::new(CircleElement::new(0)),
      Box::new(PointElement::new(1)),
      Box::new(SquareElement::new(2)),
    ]));

  println(&visitor_elements);

  visitor_elements.borrow().iter().enumerate().for_each(
    |(index, visitor_element)| {
      visitor_element.translate((index + 1) as f64, 0.)
    },
  );

  println(&visitor_elements);

  let cluster_visitor = ClusterVisitor::new(visitor_elements.clone());

  visitor_elements
    .borrow()
    .iter()
    .for_each(|visitor_element| {
      visitor_element.accept_visitor(&cluster_visitor)
    });

  println(&visitor_elements);
}

fn println(visitor_elements: &Rc<RefCell<Vec<Box<dyn VisitorElement>>>>) {
  visitor_elements
    .borrow()
    .iter()
    .for_each(|visitor_element| print!("{visitor_element} "));

  println!();
}
