//==============================================================================
//! Demonstration of the "Visitor" pattern.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-17
//! - Updated: 2024-09-17
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

mod element;
mod visitor;

use self::element::circle_element::CircleElement;
use self::element::square_element::SquareElement;
use self::visitor::cluster_visitor::ClusterVisitor;
use self::visitor::visitor_element::VisitorElement;
use std::cell::RefCell;
use std::rc::Rc;

pub fn example() {
  let visitor_elements: Vec<Rc<RefCell<dyn VisitorElement>>> = vec![
    Rc::new(RefCell::new(CircleElement::new(1.))),
    Rc::new(RefCell::new(SquareElement::new(1.))),
  ];

  println!("=== visitor_4 ===");

  println(&visitor_elements);

  visitor_elements
    .iter()
    .for_each(|element| element.borrow_mut().translate(1., 0.));

  println(&visitor_elements);

  let average_visitor = ClusterVisitor::new(&visitor_elements);

  visitor_elements.iter().for_each(|visitor_element| {
    visitor_element
      .borrow_mut()
      .accept_visitor(&average_visitor)
  });

  println(&visitor_elements);
}

fn println(visitor_elements: &[Rc<RefCell<dyn VisitorElement>>]) {
  visitor_elements
    .iter()
    .for_each(|visitor_element| print!("{} ", visitor_element.borrow()));

  println!();
}
