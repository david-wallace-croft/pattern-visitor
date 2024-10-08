use super::super::element::circle_element::CircleElement;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::super::element::Element;

pub trait ScaleElement: Element {
  fn scale(
    &mut self,
    scaling_factor: f64,
  );
}

impl ScaleElement for CircleElement {
  fn scale(
    &mut self,
    scaling_factor: f64,
  ) {
    self.radius *= scaling_factor;
  }
}

impl ScaleElement for PointElement {
  fn scale(
    &mut self,
    _scaling_factor: f64,
  ) {
    // Calling the scale method on a PointElement is pointless ;)
  }
}

impl ScaleElement for SquareElement {
  fn scale(
    &mut self,
    scaling_factor: f64,
  ) {
    self.half_height *= scaling_factor;
  }
}
