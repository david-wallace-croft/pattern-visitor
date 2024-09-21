use super::element::circle_element::CircleElement;
use super::element::point_element::PointElement;
use super::element::square_element::SquareElement;

pub mod scale_visitor;
pub mod visitor_acceptor;
pub mod visitor_element;

pub trait Visitor {
  fn visit_circle_element_f64(
    &self,
    circle_element: &mut CircleElement<f64>,
  );

  fn visit_circle_element_isize(
    &self,
    circle_element: &mut CircleElement<isize>,
  );

  fn visit_point_element_f64(
    &self,
    point_element: &mut PointElement<f64>,
  );

  fn visit_point_element_isize(
    &self,
    point_element: &mut PointElement<isize>,
  );

  fn visit_square_element_f64(
    &self,
    square_element: &mut SquareElement<f64>,
  );

  fn visit_square_element_isize(
    &self,
    square_element: &mut SquareElement<isize>,
  );
}
