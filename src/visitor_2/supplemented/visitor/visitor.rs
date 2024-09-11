use super::super::super::original::visitor::scale_visitor::ScaleVisitor;
use super::super::super::original::visitor::Visitor as OriginalVisitor;
use super::super::element::hexagon_element::HexagonElement;

pub trait Visitor: OriginalVisitor {
  fn visit_hexagon_element(
    &self,
    hexagon_element: &mut HexagonElement,
  );
}

impl Visitor for ScaleVisitor {
  fn visit_hexagon_element(
    &self,
    hexagon_element: &mut HexagonElement,
  ) {
    hexagon_element.circumcircle_radius *= self.scaling_factor;
  }
}
