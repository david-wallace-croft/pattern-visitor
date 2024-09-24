use self::scale_element::ScaleElement;
use super::element::circle_element::CircleElement;
use super::element::point_element::PointElement;
use super::element::square_element::SquareElement;

pub mod scale_element;

pub trait MixinElement: ScaleElement {}

impl MixinElement for CircleElement {}

impl MixinElement for PointElement {}

impl MixinElement for SquareElement {}
