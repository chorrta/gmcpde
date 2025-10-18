//#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]
//#![warn(clippy::nursery)]
mod compute;
mod geometry;
mod ui;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{geometry::*, ui::draw::draw_parametric};
    use cgmath::Point2;

    #[test]
    fn create_composite_lines() {
        let mut pc = ParametricComposite::new();
        pc.push_line(Point2::new(0f32, 0f32), Point2::new(5f32, 10f32))
            .unwrap();
        if let Geom2D::Line(line) = pc.into_iter().nth(0).unwrap() {
            assert_eq!(line.point_along(0.5f32), Point2::new(2.5f32, 5f32))
        }
    }

    #[test]
    fn draw_composite_lines() {
        let mut pc = ParametricComposite::new();
        pc.push_line(Point2::new(10f32, 0f32), Point2::new(50f32, 99f32))
            .unwrap();
        pc.push_line(Point2::new(30f32, 50f32), Point2::new(70f32, 50f32))
            .unwrap();
        pc.push_line(Point2::new(50f32, 99f32), Point2::new(90f32, 0f32))
            .unwrap();
        draw_parametric(&pc).unwrap()
    }
}
