use super::super::element::circle_element::CircleElement;
use super::super::element::element_accessor::ElementAccessor;
use super::super::element::point_element::PointElement;
use super::super::element::square_element::SquareElement;
use super::visitor_element::VisitorElement;
use super::Visitor;
use ::std::cell::RefCell;
use ::std::rc::Rc;

pub struct ClusterVisitor {
  visitor_elements: Rc<RefCell<Vec<Box<dyn VisitorElement>>>>,
}

impl ClusterVisitor {
  pub fn average_center(
    &self,
    excluded_element_accessor: &dyn ElementAccessor,
  ) -> Option<(f64, f64)> {
    let excluded_id: usize = excluded_element_accessor.get_id();

    self
      .visitor_elements
      .borrow()
      .iter()
      .filter(|visitor_element| visitor_element.get_id() != excluded_id)
      .map(|visitor_element| {
        (
          visitor_element.get_center_x(),
          visitor_element.get_center_y(),
          1,
        )
      })
      .reduce(|(sum_x, sum_y, count), (x, y, _)| {
        (sum_x + x, sum_y + y, count + 1)
      })
      .map(|(sum_x, sum_y, count)| (sum_x / count as f64, sum_y / count as f64))
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
    let average_center_option: Option<(f64, f64)> =
      self.average_center(circle_element);

    if let Some((average_center_x, average_center_y)) = average_center_option {
      circle_element.set_center_x(average_center_x);

      circle_element.set_center_y(average_center_y);
    }
  }

  fn visit_point_element(
    &self,
    point_element: &PointElement,
  ) {
    let average_center_option: Option<(f64, f64)> =
      self.average_center(point_element);

    if let Some((average_center_x, average_center_y)) = average_center_option {
      point_element.set_x(average_center_x);

      point_element.set_y(average_center_y);
    }
  }

  fn visit_square_element(
    &self,
    square_element: &SquareElement,
  ) {
    let average_center_option: Option<(f64, f64)> =
      self.average_center(square_element);

    if let Some((average_center_x, average_center_y)) = average_center_option {
      square_element.set_center_x(average_center_x);

      square_element.set_center_y(average_center_y);
    }
  }
}
