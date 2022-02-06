use crate::components::point::Point;

use super::shape::ColorRGBA;

pub struct Text {
    pub content: String,
    pub position: Point,
    pub font: String,
    pub color: ColorRGBA,
}
