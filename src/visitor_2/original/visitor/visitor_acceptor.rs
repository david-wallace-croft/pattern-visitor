use super::super::element::circle_element::CircleElement;
use super::super::element::square_element::SquareElement;
use super::Visitor;
use crate::visitor_2::original::element::point_element::PointElement;

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
