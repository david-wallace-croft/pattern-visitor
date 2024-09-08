use super::circle_element::CircleElement;
use super::square_element::SquareElement;

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
