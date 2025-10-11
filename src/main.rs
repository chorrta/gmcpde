//#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]
//#![warn(clippy::nursery)]
use cgmath::Point2;

use crate::geometry::{ClampedF32, Parametric};

mod compute;
mod geometry;
mod ui;

fn main() {
    println!("Hello, world!");
    //    ui::draw::draw_circle().unwrap();
    let line = geometry::Line::new(Point2::new(2f32, 3f32), Point2::new(5f32, 10f32)).unwrap();
    let points = line.points();
    println!("The x coordinate of the points is: {}", points[0].x);
    println!(
        "Midpoint x: {}",
        line.point_along(ClampedF32::new(0.5f32).unwrap()).x
    );
}
