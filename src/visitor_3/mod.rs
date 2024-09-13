use self::element::circle_element::CircleElement;
use self::element::hexagon_element::HexagonElement;
use self::element::square_element::SquareElement;
use self::element::Element;

mod element;

pub fn example() {
  let mut elements: Vec<Box<dyn Element>> = vec![
    Box::new(CircleElement::new(1.)),
    Box::new(SquareElement::new(1.)),
    Box::new(HexagonElement::new(1.)),
  ];

  elements
    .iter_mut()
    .for_each(|element| element.translate(1., 0.));
}
