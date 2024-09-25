//==============================================================================
//! The Element trait and its implementations.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-19
//! - Updated: 2024-09-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

use self::element_accessor::ElementAccessor;
use ::std::fmt::Display;

// Element modules are public
pub mod circle_element;
pub mod element_accessor;
pub mod point_element;
pub mod square_element;

pub trait Element: Display + ElementAccessor {
  // An Element might have some functionality common to all implementations
  fn translate(
    &self,
    offset_x: f64,
    offset_y: f64,
  );
}
