use super::super::super::original::visitor::Visitor;
use super::super::element::hexagon_element::HexagonElement;

pub trait ExtendedVisitor: Visitor {
  fn visit_hexagon_element(
    &self,
    hexagon_element: &mut HexagonElement,
  );
}
