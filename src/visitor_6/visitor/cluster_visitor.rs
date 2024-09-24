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
  fn cluster(
    &self,
    element: &dyn Element,
  ) {
    let id: usize = element.get_id();

    let count: f64 = (self.visitor_elements.borrow().len() - 1) as f64;

    let average_center_option = self
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
      .map(|(sum_x, sum_y)| (sum_x / count, sum_y / count));

    if let Some((average_x, average_y)) = average_center_option {
      element.set_center_x(average_x);

      element.set_center_y(average_y);
    }
  }

  pub fn new(
    visitor_elements: Rc<RefCell<Vec<Box<dyn VisitorElement>>>>
  ) -> Self {
    Self {
      visitor_elements,
    }
  }
}

impl Visitor for ClusterVisitor {
  fn visit_circle_element(
    &self,
    circle_element: &CircleElement,
  ) {
    self.cluster(circle_element);
  }

  fn visit_point_element(
    &self,
    point_element: &PointElement,
  ) {
    self.cluster(point_element);
  }

  fn visit_square_element(
    &self,
    square_element: &SquareElement,
  ) {
    self.cluster(square_element);
  }
}
