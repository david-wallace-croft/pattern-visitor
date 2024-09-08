use super::circle_element::CircleElement;
use super::square_element::SquareElement;

pub trait Visitor {
  fn visit_circle_element_f64(
    &self,
    circle_element: &mut CircleElement<f64>,
  );

  fn visit_circle_element_isize(
    &self,
    circle_element: &mut CircleElement<isize>,
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
