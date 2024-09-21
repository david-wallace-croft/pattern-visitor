use super::super::element::circle_element::CircleElement;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::Visitor;

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
  fn visit_circle_element_f64(
    &self,
    circle_element: &mut CircleElement<f64>,
  ) {
    circle_element.radius *= self.scaling_factor;
  }

  fn visit_circle_element_isize(
    &self,
    circle_element: &mut CircleElement<isize>,
  ) {
    circle_element.radius = ((circle_element.radius as f64)
      * self.scaling_factor)
      .round_ties_even() as isize;
  }

  fn visit_point_element_f64(
    &self,
    _point_element: &mut PointElement<f64>,
  ) {
    // Calling ScaleVisitor on a PointElement is pointless ;)
  }

  fn visit_point_element_isize(
    &self,
    _point_element: &mut PointElement<isize>,
  ) {
    // Calling ScaleVisitor on a PointElement is pointless ;)
  }

  fn visit_square_element_f64(
    &self,
    square_element: &mut SquareElement<f64>,
  ) {
    square_element.half_height *= self.scaling_factor;
  }

  fn visit_square_element_isize(
    &self,
    square_element: &mut SquareElement<isize>,
  ) {
    square_element.half_height = ((square_element.half_height as f64)
      * self.scaling_factor)
      .round_ties_even() as isize;
  }
}
