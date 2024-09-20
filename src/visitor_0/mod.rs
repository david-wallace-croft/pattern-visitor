//==============================================================================
//! Demonstration of the "Visitor" pattern.
//!
//! - The "element" module provides the "Element" trait and its implementations.
//! - Trait Element defines methods common to all of its implementations.
//!
//! - The "visitor" module provides the "Visitor" trait and its implementations.
//! - A Visitor implementation adds functionality to an Element implementation.
//! - Trait Visitor defines a visit_*_element() method for each Element.
//! - The Visitor method is used instead of adding another method to Element.
//! - The visitor module also provides two adapter traits.
//!
//! - Adapter trait "VisitorAcceptor" defines the accept_visitor() method.
//! - There is a VisitorAcceptor implementation for each Element.
//! - Using VisitorAcceptor keeps trait Element independent of trait Visitor.
//! - This permits Element to be used with Visitor traits from other crates.
//!
//! - Adapter trait "VisitorElement" extends traits Element and VisitorAcceptor.
//! - There is a VisitorElement implementation for each Element.
//! - All Element methods can be called on a VisitorElement.
//! - All Visitor implementations can visit a VisitorElement.
//!
//! - The element module is independent of the visitor module.
//! - No changes to the element module are required when a Visitor is added.
//! - The visitor module imports the element module.
//! - Changes to the visitor module are required when an Element is added.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-08
//! - Updated: 2024-09-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

mod element;
mod visitor;

use self::element::circle_element::CircleElement;
use self::element::square_element::SquareElement;
use self::visitor::scale_visitor::ScaleVisitor;
use self::visitor::visitor_element::VisitorElement;

pub fn example() {
  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::new(1.)),
    // TODO: add PointElement
    Box::new(SquareElement::new(1.)),
  ];

  println!("=== visitor_0 ===");

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
