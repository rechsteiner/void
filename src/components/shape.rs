use perlin2d::PerlinNoise2D;
use std::fmt;
use web_sys::console;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Shape {
    pub vertices: Vec<Point>,
    pub color: ColorRGBA, //Mosly for debug
    pub is_sensor: bool,
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

// The idea behind Polygon is to have our own name for Vec<Point>
// with associated generator functions to get circles, triangles etc.

pub struct Polygon(Vec<Point>);

impl Polygon {
    /// Generate a polygon in the shape of a planetoid.
    /// Using different noise seeds will generate different types of terrains.
    pub fn planetoid(radius: f32, resolution: i32, noise_seed: i32) -> Vec<Point> {
        let mut polygon: Vec<Point> = vec![];

        // Perlin noise is a continuous type of noise, which fits perfectly for generating terrains.
        // In this case we use the noise to offset points on a sphere, to create an irregular terrain.
        let perlin = PerlinNoise2D::new(6, 0.03, 0.4, 1.0, 1.5, (0.30, 0.30), 1.0, noise_seed);

        for i in 0..resolution {
            let pi_2: f64 = 3.1415 * 2.0;
            let progress: f64 = (i as f64) * pi_2 / (resolution as f64);

            let coord_x = progress.sin();
            let coord_y = progress.cos();

            let noise_offset = perlin.get_noise(coord_x, coord_y);
            console::log_1(&noise_offset.into());

            polygon.push(Point {
                x: (noise_offset * coord_x) as f32 * radius,
                y: (noise_offset * coord_y) as f32 * radius,
            });
        }

        polygon
    }
}
