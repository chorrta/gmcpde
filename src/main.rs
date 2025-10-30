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
    use crate::{compute::*, geometry::*, ui::draw};
    use cgmath::Point2;
    use plotters::style::full_palette;

    #[test]
    fn create_composite_lines() {
        let mut pc = ParametricComposite::new();
        pc.push_line(Point2::new(0f32, 0f32), Point2::new(5f32, 10f32))
            .unwrap();
        let Geom2D::Line(line) = pc.into_iter().nth(0).unwrap();
        assert_eq!(line.point_along(0.5f32), Point2::new(2.5f32, 5f32))
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
        let canvas = draw::CanvasPDE2D::new("outputs/tests/test_comp_lines.svg", [1000, 500]);
        canvas
            .draw_parametric(&pc, 10, full_palette::AMBER)
            .unwrap();
    }
    #[test]
    fn draw_result_with_geom() {
        let pc = letter_a_from_lines();
        let resolution: [usize; 2] = [1000, 500];
        let canvas = draw::CanvasPDE2D::new("outputs/tests/test_result_w_geom.svg", resolution);
        let renderer = MonteCarloPDE2D::new(resolution, 1, 100, 2f32.powi(-20));
        let result = renderer.find_pde(&pc, Method2D::Laplacian);
        canvas.draw_result(&result).unwrap();
        canvas
            .draw_parametric(&pc, 10, full_palette::BLACK)
            .unwrap();
        canvas.present().unwrap();
    }

    #[test]
    fn line_queries() {
        let pc = letter_a_from_lines();
        let query: [Point2<f32>; 2] = [Point2::new(0.5f32, 0.4f32), Point2::new(0.7f32, 10f32)];
        println!(
            "Closest point is at x: {}, y : {}",
            pc.closest_point(query[0]).expect("").x,
            pc.closest_point(query[0]).expect("").y
        );
        assert_eq!(pc.closest_point(query[0]).expect("").x, 0.5f32);
        assert_eq!(pc.closest_point(query[0]).expect("").y, 0.5f32);
        assert_eq!(pc.closest_point(query[1]).expect("").x, 0.5f32);
        assert_eq!(pc.closest_point(query[1]).expect("").y, 1f32);
    }
}
