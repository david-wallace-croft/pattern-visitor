use super::item::Item;
use super::visitor::Visitor;
use std::marker::PhantomData;

pub struct HalfValueVisitor<T> {
  data: PhantomData<T>,
}

impl<T> Default for HalfValueVisitor<T> {
  fn default() -> Self {
    Self {
      data: PhantomData,
    }
  }
}

impl Visitor<f64> for HalfValueVisitor<f64> {
  fn visit(&self, item: &mut Item<f64>) {
    item.value /= 2.;
  }
}

impl Visitor<usize> for HalfValueVisitor<usize> {
  fn visit(&self, item: &mut Item<usize>) {
    item.value /= 2;
  }
}
