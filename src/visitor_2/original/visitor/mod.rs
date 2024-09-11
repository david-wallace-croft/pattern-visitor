use super::element::circle_element::CircleElement;
use super::element::square_element::SquareElement;

pub mod scale_visitor;
pub mod visitor_acceptor;
pub mod visitor_element;

pub trait Visitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  );

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  );
}
