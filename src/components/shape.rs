use std::fmt;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Shape {
    pub vertices: Vec<Point>,
    // pub width: f32,
    // pub height: f32,
    pub color: ColorRGBA, //Mosly for debug
}

pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl fmt::Display for ColorRGBA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
