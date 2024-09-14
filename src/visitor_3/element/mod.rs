pub mod circle_element;
pub mod hexagon_element;
pub mod square_element;

use std::fmt::Display;

pub trait Element: Display {
  fn get_center_x(&self) -> f64;

  fn get_center_y(&self) -> f64;

  fn get_circumcircle_radius(&self) -> f64;

  fn get_id(&self) -> usize;

  fn translate(
    &mut self,
    offset_x: f64,
    offset_y: f64,
  );
}
