use crate::visitor_2::extension::extended_visitor::extended_visitor::ExtendedVisitor;
use crate::visitor_2::extension::extended_visitor::extended_visitor_acceptor::ExtendedVisitorAcceptor;
use crate::visitor_2::original::element::circle_element::CircleElement;
use crate::visitor_2::original::element::Element;

pub mod element;
pub mod extended_visitor;

pub trait ExtendedElement: Element + ExtendedVisitorAcceptor {}

impl ExtendedVisitorAcceptor for CircleElement {
  fn accept_extended_visitor(
    &mut self,
    _extended_visitor: &dyn ExtendedVisitor,
  ) {
    todo!()
  }
}

impl ExtendedElement for CircleElement {}
