use super::super::element::circle_element::CircleElement;
use super::super::element::hexagon_element::HexagonElement;
use super::super::element::square_element::SquareElement;
use super::super::element::Element;
use super::visitor_element::VisitorElement;
use super::Visitor;

pub struct ClusterVisitor<'a> {
  elements: &'a Vec<Box<dyn VisitorElement>>,
}

impl<'a> ClusterVisitor<'a> {
  fn cluster(
    &self,
    _element: &mut dyn Element,
  ) {
    // TODO
  }

  pub fn new(elements: &'a Vec<Box<dyn VisitorElement>>) -> Self {
    Self {
      elements,
    }
  }
}

impl Visitor for ClusterVisitor<'_> {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  ) {
    self.cluster(circle_element);
  }

  fn visit_hexagon_element(
    &self,
    hexagon_element: &mut HexagonElement,
  ) {
    self.cluster(hexagon_element);
  }

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  ) {
    self.cluster(square_element);
  }
}
