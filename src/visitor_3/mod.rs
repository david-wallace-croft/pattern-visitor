use self::cluster_element::ClusterElement;
use self::element::circle_element::CircleElement;
use self::element::hexagon_element::HexagonElement;
use self::element::square_element::SquareElement;

mod cluster_element;
mod element;

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
}
