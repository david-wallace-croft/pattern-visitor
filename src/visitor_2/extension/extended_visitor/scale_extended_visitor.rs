use super::super::super::original::element::circle_element::CircleElement;
use super::super::super::original::element::square_element::SquareElement;
use super::super::super::original::visitor::scale_visitor::ScaleVisitor;
use super::super::super::original::visitor::Visitor;
use super::super::element::hexagon_element::HexagonElement;
use super::super::extended_visitor::extended_visitor::ExtendedVisitor;

pub struct ScaleExtendedVisitor {
  scale_visitor: ScaleVisitor,
  scaling_factor: f64,
}

impl ScaleExtendedVisitor {
  pub fn new(scaling_factor: f64) -> Self {
    Self {
      scale_visitor: ScaleVisitor::new(scaling_factor),
      scaling_factor,
    }
  }
}

impl Visitor for ScaleExtendedVisitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  ) {
    self
      .scale_visitor
      .visit_circle_element(circle_element);
  }

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  ) {
    self
      .scale_visitor
      .visit_square_element(square_element);
  }
}

impl ExtendedVisitor for ScaleExtendedVisitor {
  fn visit_hexagon_element(
    &self,
    hexagon_element: &mut HexagonElement,
  ) {
    hexagon_element.circumcircle_radius *= self.scaling_factor;
  }
}
