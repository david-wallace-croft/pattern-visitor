use super::value_element::ValueElement;
use super::visitor_element::VisitorElement;
use std::fmt::Debug;

pub trait Element: Debug + ValueElement + VisitorElement {}
