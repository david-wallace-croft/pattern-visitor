use super::extended_visitor::ExtendedVisitor;

pub trait ExtendedVisitorAcceptor {
  fn accept_extended_visitor(
    &mut self,
    extended_visitor: &dyn ExtendedVisitor,
  );
}
