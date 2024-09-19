use super::super::element::circle_element::CircleElement;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::visitor_element::VisitorElement;
use super::Visitor;

pub struct ClusterVisitor {
  average_center_x: f64,
  average_center_y: f64,
}

impl ClusterVisitor {
  pub fn new(
    index: usize,
    visitor_elements: &[Box<dyn VisitorElement>],
  ) -> Self {
    let sum_center_x: f64 = visitor_elements
      .iter()
      .enumerate()
      .filter(|(j, _)| *j != index)
      .map(|(_, visitor_element)| visitor_element.get_center_x())
      .sum();

    let sum_center_y: f64 = visitor_elements
      .iter()
      .enumerate()
      .filter(|(j, _)| *j != index)
      .map(|(_, visitor_element)| visitor_element.get_center_y())
      .sum();

    let length: usize = visitor_elements.len();

    let average_center_x: f64 = sum_center_x / ((length - 1) as f64);

    let average_center_y: f64 = sum_center_y / ((length - 1) as f64);

    Self {
      average_center_x,
      average_center_y,
    }
  }
}

impl Visitor for ClusterVisitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  ) {
    circle_element.center_x = self.average_center_x;

    circle_element.center_y = self.average_center_y;
  }

  fn visit_point_element(
    &self,
    point_element: &mut PointElement,
  ) {
    point_element.center_x = self.average_center_x;

    point_element.center_y = self.average_center_y;
  }

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  ) {
    square_element.center_x = self.average_center_x;

    square_element.center_y = self.average_center_y;
  }
}
