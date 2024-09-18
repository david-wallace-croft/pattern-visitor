use super::super::element::circle_element::CircleElement;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::visitor_element::VisitorElement;
use super::Visitor;
use ::std::cell::RefCell;
use ::std::collections::VecDeque;
use ::std::rc::Rc;

pub struct ClusterVisitor {
  visitor_elements: Rc<RefCell<VecDeque<Box<dyn VisitorElement>>>>,
}

impl ClusterVisitor {
  pub fn new(
    visitor_elements: Rc<RefCell<VecDeque<Box<dyn VisitorElement>>>>
  ) -> Self {
    Self {
      visitor_elements,
    }
  }

  fn average_center_x(&self) -> f64 {
    let sum_x: f64 = self
      .visitor_elements
      .borrow()
      .iter()
      .map(|visitor_element| visitor_element.get_center_x())
      .sum();

    sum_x / (self.visitor_elements.borrow().len() as f64)
  }

  fn average_center_y(&self) -> f64 {
    let sum_y: f64 = self
      .visitor_elements
      .borrow()
      .iter()
      .map(|visitor_element| visitor_element.get_center_y())
      .sum();

    sum_y / (self.visitor_elements.borrow().len() as f64)
  }
}

impl Visitor for ClusterVisitor {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  ) {
    circle_element.center_x = self.average_center_x();

    circle_element.center_y = self.average_center_y();
  }

  fn visit_point_element(
    &self,
    point_element: &mut PointElement,
  ) {
    point_element.center_x = self.average_center_x();

    point_element.center_y = self.average_center_y();
  }

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  ) {
    square_element.center_x = self.average_center_x();

    square_element.center_y = self.average_center_y();
  }
}
