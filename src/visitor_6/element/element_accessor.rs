//==============================================================================
//! Accessor (getter) methods for an Element.
//!
//! # Metadata
//! - Author: [`David Wallace Croft`]
//! - Copyright: &copy; 2024 [`CroftSoft Inc`]
//! - Created: 2024-09-25
//! - Updated: 2024-09-25
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
//==============================================================================

pub trait ElementAccessor {
  fn get_center_x(&self) -> f64;

  fn get_center_y(&self) -> f64;

  fn get_id(&self) -> usize;
}
