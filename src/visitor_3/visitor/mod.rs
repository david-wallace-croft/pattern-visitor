//==============================================================================
//! The Visitor trait, its implementations, and adapter traits.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-09
//! - Updated: 2024-09-09
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::element::circle_element::CircleElement;
use super::element::square_element::SquareElement;

pub mod scale_visitor;
pub mod visitor_acceptor;
pub mod visitor_element;

pub trait Visitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  );

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  );
}