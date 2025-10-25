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

    fn letter_a_from_lines() -> ParametricComposite {
        let mut pc = ParametricComposite::new();
        pc.push_line(Point2::new(0.1f32, 0f32), Point2::new(0.50f32, 1f32))
            .unwrap();
        pc.push_line(Point2::new(0.30f32, 0.50f32), Point2::new(0.70f32, 0.50f32))
            .unwrap();
        pc.push_line(Point2::new(0.50f32, 1f32), Point2::new(0.90f32, 0f32))
            .unwrap();
        pc
    }

    #[test]
    fn draw_composite_lines() {
        let pc = letter_a_from_lines();
        draw_parametric(&pc).unwrap()
    }

    #[test]
    fn line_queries() {
        let pc = letter_a_from_lines();
        let query: Point2<f32> = Point2::new(0.2f32, 0.2f32);
        println!(
            "Closest point is at x: {}, y : {}",
            pc.closest_point(query).expect("").x,
            pc.closest_point(query).expect("").y
        )
    }
}
