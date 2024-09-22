use super::element::circle_element::CircleElement;
use super::element::point_element::PointElement;
use super::element::square_element::SquareElement;

pub mod scale_visitor;
pub mod visitor_acceptor;
pub mod visitor_element;

pub trait Visitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  );

  fn visit_point_element(
    &self,
    point_element: &mut PointElement,
  );

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  );
}
