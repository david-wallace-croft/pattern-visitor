//==============================================================================
//! The Visitor trait, its implementations, and adapter traits.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-18
//! - Updated: 2024-09-18
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use super::element::circle_element::CircleElement;
use super::element::point_element::PointElement;
use super::element::square_element::SquareElement;

pub mod cluster_visitor;
pub mod visitor_acceptor;
pub mod visitor_element;

pub trait Visitor {
  fn visit_circle_element(
    &self,
    circle_element: &CircleElement,
  );

  fn visit_point_element(
    &self,
    point_element: &PointElement,
  );

  fn visit_square_element(
    &self,
    square_element: &SquareElement,
  );
}
