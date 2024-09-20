use super::super::element::circle_element::CircleElement;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::visitor_element::VisitorElement;
use super::Visitor;
use crate::visitor_6::element::Element;
use ::std::cell::RefCell;
use ::std::rc::Rc;

pub struct ClusterVisitor {
  visitor_elements: Rc<RefCell<Vec<Box<dyn VisitorElement>>>>,
}

impl ClusterVisitor {
  pub fn new(
    visitor_elements: Rc<RefCell<Vec<Box<dyn VisitorElement>>>>
  ) -> Self {
    Self {
      visitor_elements,
    }
  }

  fn average_center(
    &self,
    id: usize,
  ) -> Option<(f64, f64)> {
    let count: f64 = (self.visitor_elements.borrow().len() - 1) as f64;

    self
      .visitor_elements
      .borrow()
      .iter()
      .filter(|visitor_element| visitor_element.get_id() != id)
      .map(|visitor_element| {
        (
          visitor_element.get_center_x(),
          visitor_element.get_center_y(),
        )
      })
      .reduce(|(sum_x, sum_y), (x, y)| (sum_x + x, sum_y + y))
      .map(|(sum_x, sum_y)| (sum_x / count, sum_y / count))
  }
}

impl Visitor for ClusterVisitor {
  fn visit_circle_element(
    &self,
    circle_element: &CircleElement,
  ) {
    let id: usize = circle_element.get_id();

    if let Some((average_x, average_y)) = self.average_center(id) {
      circle_element.set_center_x(average_x);

      circle_element.set_center_y(average_y);
    }
  }

  fn visit_point_element(
    &self,
    point_element: &PointElement,
  ) {
    let id: usize = point_element.get_id();

    if let Some((average_x, average_y)) = self.average_center(id) {
      point_element.set_center_x(average_x);

      point_element.set_center_y(average_y);
    }
  }

  fn visit_square_element(
    &self,
    square_element: &SquareElement,
  ) {
    let id: usize = square_element.get_id();

    if let Some((average_x, average_y)) = self.average_center(id) {
      square_element.set_center_x(average_x);

      square_element.set_center_y(average_y);
    }
  }
}
