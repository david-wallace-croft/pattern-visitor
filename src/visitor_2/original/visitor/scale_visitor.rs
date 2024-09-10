use super::super::element::circle_element::CircleElement;
use super::super::element::square_element::SquareElement;
use super::super::visitor::Visitor;

pub struct ScaleVisitor {
  scaling_factor: f64,
}

impl ScaleVisitor {
  pub fn new(scaling_factor: f64) -> Self {
    Self {
      scaling_factor,
    }
  }
}

impl Visitor for ScaleVisitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  ) {
    circle_element.radius *= self.scaling_factor;
  }

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  ) {
    square_element.half_height *= self.scaling_factor;
  }
}
