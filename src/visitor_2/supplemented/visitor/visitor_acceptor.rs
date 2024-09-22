use super::super::super::original::element::circle_element::CircleElement;
use super::super::super::original::element::point_element::PointElement;
use super::super::super::original::element::square_element::SquareElement;
use super::super::element::hexagon_element::HexagonElement;
use super::Visitor;

pub trait VisitorAcceptor {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  );
}

impl VisitorAcceptor for CircleElement {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_circle_element(self);
  }
}

impl VisitorAcceptor for HexagonElement {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_hexagon_element(self);
  }
}

impl VisitorAcceptor for PointElement {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_point_element(self);
  }
}

impl VisitorAcceptor for SquareElement {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_square_element(self);
  }
}
