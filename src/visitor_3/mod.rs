//==============================================================================
//! Comparison of the "Visitor" pattern to the "Mixin" pattern.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-16
//! - Updated: 2024-09-16
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

mod element;
mod mixin;
mod visitor;

use self::element::circle_element::CircleElement;
use self::element::point_element::PointElement;
use self::element::square_element::SquareElement;
use self::visitor::scale_visitor::ScaleVisitor;
use self::visitor::visitor_element::VisitorElement;
use crate::visitor_3::mixin::MixinElement;

pub fn example() {
  example_mixin();

  example_visitor();
}

fn example_mixin() {
  let mut mixin_elements: Vec<Box<dyn MixinElement>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(PointElement::default()),
    Box::new(SquareElement::new(1.)),
  ];

  println!("=== visitor_3 mixin ===");

  println_mixin_elements(&mixin_elements);

  mixin_elements
    .iter_mut()
    .for_each(|element| element.translate(1., 0.));

  println_mixin_elements(&mixin_elements);

  mixin_elements
    .iter_mut()
    .for_each(|mixin_element| mixin_element.scale(2.));

  println_mixin_elements(&mixin_elements);
}

fn example_visitor() {
  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(PointElement::default()),
    Box::new(SquareElement::new(1.)),
  ];

  println!("=== visitor_3 visitor ===");

  println_visitor_elements(&visitor_elements);

  visitor_elements
    .iter_mut()
    .for_each(|element| element.translate(1., 0.));

  println_visitor_elements(&visitor_elements);

  let scale_visitor = ScaleVisitor::new(2.);

  visitor_elements
    .iter_mut()
    .for_each(|visitor_element| visitor_element.accept_visitor(&scale_visitor));

  println_visitor_elements(&visitor_elements);
}

fn println_mixin_elements(mixin_elements: &[Box<dyn MixinElement>]) {
  mixin_elements
    .iter()
    .for_each(|mixin_element| print!("{mixin_element} "));

  println!();
}

fn println_visitor_elements(visitor_elements: &[Box<dyn VisitorElement>]) {
  visitor_elements
    .iter()
    .for_each(|visitor_element| print!("{visitor_element} "));

  println!();
}
