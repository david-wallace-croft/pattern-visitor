use self::cluster_element::ClusterElement;
use self::element::circle_element::CircleElement;
use self::element::hexagon_element::HexagonElement;
use self::element::square_element::SquareElement;
use self::visitor::cluster_visitor::ClusterVisitor;
use self::visitor::visitor_element::VisitorElement;

mod cluster_element;
mod element;
mod visitor;

pub fn example() {
  let mut cluster_elements: Vec<Box<dyn ClusterElement>> = vec![
    Box::new(CircleElement::new(0, 1.)),
    Box::new(HexagonElement::new(1, 1.)),
    Box::new(SquareElement::new(2, 1.)),
  ];

  cluster_elements
    .iter_mut()
    .for_each(|cluster_element| cluster_element.translate(1., 0.));

  // Compilation for the following fails with this error message:
  // "cannot borrow `cluster_elements` as immutable because it is also borrowed
  // as mutable"
  //
  // cluster_elements
  //   .iter_mut()
  //   .for_each(|cluster_element| cluster_element.cluster(&cluster_elements));

  let mut visitor_elements: Vec<Box<dyn VisitorElement>> = vec![
    Box::new(CircleElement::new(0, 1.)),
    Box::new(HexagonElement::new(1, 1.)),
    Box::new(SquareElement::new(2, 1.)),
  ];

  visitor_elements
    .iter_mut()
    .for_each(|visitor_element| visitor_element.translate(1., 0.));

  let _cluster_visitor = ClusterVisitor::new(&visitor_elements);

  // Compilation for the following fails with this error message:
  // cannot borrow `visitor_elements` as mutable because it is also borrowed as
  // immutable
  //
  // visitor_elements.iter_mut().for_each(|visitor_element| {
  //   visitor_element.accept_visitor(&cluster_visitor)
  // });
}
