use super::super::element::circle_element::CircleElement;
use super::super::element::square_element::SquareElement;
use super::Visitor;
use crate::visitor_4::visitor::visitor_element::VisitorElement;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ClusterVisitor<'a> {
  visitor_elements: &'a Vec<Rc<RefCell<dyn VisitorElement>>>,
}

impl<'a> ClusterVisitor<'a> {
  pub fn new(
    visitor_elements: &'a Vec<Rc<RefCell<dyn VisitorElement>>>
  ) -> Self {
    Self {
      visitor_elements,
    }
  }

  pub fn average_center_x(&self) -> f64 {
    let sum_x: f64 = self
      .visitor_elements
      .iter()
      .map(|visitor_element| visitor_element.borrow().get_center_x())
      .sum();

    sum_x / (self.visitor_elements.len() as f64)
  }

  pub fn average_center_y(&self) -> f64 {
    let sum_y: f64 = self
      .visitor_elements
      .iter()
      .map(|visitor_element| visitor_element.borrow().get_center_y())
      .sum();

    sum_y / (self.visitor_elements.len() as f64)
  }
}

impl Visitor for ClusterVisitor<'_> {
  fn visit_circle_element(
    &self,
    circle_element: &mut CircleElement,
  ) {
    circle_element.center_x = self.average_center_x();

    circle_element.center_y = self.average_center_y();
  }

  fn visit_square_element(
    &self,
    square_element: &mut SquareElement,
  ) {
    square_element.center_x = self.average_center_x();

    square_element.center_y = self.average_center_y();
  }
}
