use super::super::element::circle_element::CircleElement;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::Visitor;

pub trait VisitorAcceptor {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  );
}

impl VisitorAcceptor for CircleElement<f64> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_circle_element_f64(self);
  }
}

impl VisitorAcceptor for CircleElement<isize> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_circle_element_isize(self);
  }
}

impl VisitorAcceptor for PointElement<f64> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_point_element_f64(self);
  }
}

impl VisitorAcceptor for PointElement<isize> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_point_element_isize(self);
  }
}

impl VisitorAcceptor for SquareElement<f64> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_square_element_f64(self);
  }
}

impl VisitorAcceptor for SquareElement<isize> {
  fn accept_visitor(
    &mut self,
    visitor: &dyn Visitor,
  ) {
    visitor.visit_square_element_isize(self);
  }
}
