use std::fmt::Debug;

pub trait ValueItem: Debug {
  fn double_value(&mut self);
}
