use super::super::element::circle_element::CircleElement;
use super::super::element::hexagon_element::HexagonElement;
use super::super::element::square_element::SquareElement;
use super::super::element::Element;
use super::visitor_acceptor::VisitorAcceptor;

pub trait VisitorElement: Element + VisitorAcceptor {}

impl VisitorElement for CircleElement {}

impl VisitorElement for HexagonElement {}

impl VisitorElement for SquareElement {}
