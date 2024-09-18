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
use self::element::point_element::PointElement;
use self::element::square_element::SquareElement;
use self::visitor::cluster_visitor::ClusterVisitor;
use self::visitor::visitor_element::VisitorElement;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn example() {
  let visitor_elements: Rc<RefCell<VecDeque<Box<dyn VisitorElement>>>> =
    Rc::new(RefCell::new(VecDeque::new()));

  visitor_elements
    .borrow_mut()
    .push_back(Box::new(CircleElement::new(1.)));

  visitor_elements
    .borrow_mut()
    .push_back(Box::new(PointElement::default()));

  visitor_elements
    .borrow_mut()
    .push_back(Box::new(SquareElement::new(1.)));

  println!("=== visitor_4 ===");

  println(&visitor_elements);

  visitor_elements
    .borrow_mut()
    .iter_mut()
    .enumerate()
    .for_each(|(index, element)| element.translate((index + 1) as f64, 0.));

  println(&visitor_elements);

  let average_visitor = ClusterVisitor::new(visitor_elements.clone());

  let length = visitor_elements.borrow().len();

  for _ in 0..length {
    let visitor_element_option: Option<Box<dyn VisitorElement>> =
      visitor_elements.borrow_mut().pop_front();

    let mut visitor_element: Box<dyn VisitorElement> =
      visitor_element_option.unwrap();

    visitor_element.accept_visitor(&average_visitor);

    visitor_elements.borrow_mut().push_back(visitor_element);
  }

  println(&visitor_elements);
}

fn println(visitor_elements: &Rc<RefCell<VecDeque<Box<dyn VisitorElement>>>>) {
  visitor_elements
    .borrow()
    .iter()
    .for_each(|visitor_element| print!("{visitor_element} "));

  println!();
}
