use self::element::Element;
use self::visitor::visitor_acceptor::VisitorAcceptor;
use crate::visitor_2::original::element::circle_element::CircleElement;
use crate::visitor_2::original::element::square_element::SquareElement;

pub mod element;
pub mod visitor;

pub trait OriginalElement: Element + VisitorAcceptor {}

impl OriginalElement for CircleElement {}

impl OriginalElement for SquareElement {}
