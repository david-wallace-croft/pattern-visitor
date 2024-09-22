pub mod circle_element;
pub mod point_element;
pub mod square_element;

use std::fmt::Display;

pub trait Element: Display {
  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  );
}
