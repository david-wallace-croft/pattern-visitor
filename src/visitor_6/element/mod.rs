//==============================================================================
//! The Element trait and its implementations.
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

use ::std::fmt::Display;

// Element implementation modules are public
pub mod circle_element;
pub mod point_element;
pub mod square_element;

pub trait Element: Display {
  fn get_center_x(&self) -> f64;

  fn get_center_y(&self) -> f64;

  fn get_id(&self) -> usize;

  fn set_center_x(
    &self,
    center_x: f64,
  );

  fn set_center_y(
    &self,
    center_y: f64,
  );

  // An Element might have some functionality common to all implementations
  fn translate(
    &self,
    offset_x: f64,
    offset_y: f64,
  );
}
