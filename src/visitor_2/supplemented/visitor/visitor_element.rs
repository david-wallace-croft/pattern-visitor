use super::super::super::original::element::circle_element::CircleElement;
use super::super::super::original::element::point_element::PointElement;
use super::super::super::original::element::square_element::SquareElement;
use super::super::super::original::element::Element;
use super::super::element::hexagon_element::HexagonElement;
use super::visitor_acceptor::VisitorAcceptor;

pub trait VisitorElement: Element + VisitorAcceptor {}

impl VisitorElement for CircleElement {}

impl VisitorElement for HexagonElement {}

impl VisitorElement for PointElement {}

impl VisitorElement for SquareElement {}
