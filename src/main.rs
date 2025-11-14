//#![deny(clippy::all)]
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]
//#![warn(clippy::nursery)]
use cgmath::Point2;
mod compute;
mod geometry;
mod ui;

fn main() {
    const WALKS_PER_PIXEL: usize = 50;
    const MAX_WALKS: usize = 100;
    const STOPPING_TOL: f32 = 2e-10;
    const OUTPUT_PATH: &str = "outputs/first_result.bmp";
    const RESOLUTION: [usize; 2] = [2000, 2000];

    let mut pc = geometry::ParametricComposite::new();
    pc.push_line(Point2::new(0.1f32, 0f32), Point2::new(0.50f32, 1f32))
        .unwrap();
    pc.push_line(Point2::new(0.30f32, 0.50f32), Point2::new(0.70f32, 0.50f32))
        .unwrap();
    pc.push_line(Point2::new(0.50f32, 1f32), Point2::new(0.90f32, 0f32))
        .unwrap();
    let canvas = ui::draw::CanvasPDE2D::new(OUTPUT_PATH, RESOLUTION);
    let renderer =
        compute::MonteCarloPDE2D::new(RESOLUTION, WALKS_PER_PIXEL, MAX_WALKS, STOPPING_TOL);
    let result = renderer
        .find_pde(
            &pc,
            compute::Method2D::Laplacian(|p| {
                p.x + p.y + 2000000f32 / (1000f32 + p.x) * (500f32 + p.y)
            }),
        )
        .unwrap();
    canvas.draw_result(&result).unwrap();
    //canvas.draw_parametric(&pc, 1, full_palette::BLACK).unwrap();
    canvas.present().unwrap();
}

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

    fn add_letter_a_to_pc(mut pc: ParametricComposite) -> ParametricComposite {
        pc.push_line(Point2::new(0.1f32, 0f32), Point2::new(0.50f32, 1f32))
            .unwrap();
        pc.push_line(Point2::new(0.30f32, 0.50f32), Point2::new(0.70f32, 0.50f32))
            .unwrap();
        pc.push_line(Point2::new(0.50f32, 1f32), Point2::new(0.90f32, 0f32))
            .unwrap();
        pc
    }
    fn add_border_to_pc(mut pc: ParametricComposite) -> ParametricComposite {
        const MIN_VALUE_FOR_SQUARE: f32 = 0f32;
        const MAX_VALUE_FOR_SQUARE: f32 = 1f32;
        let corner1 = Point2::new(MIN_VALUE_FOR_SQUARE, MIN_VALUE_FOR_SQUARE);
        let corner2 = Point2::new(MAX_VALUE_FOR_SQUARE, MIN_VALUE_FOR_SQUARE);
        let corner3 = Point2::new(MIN_VALUE_FOR_SQUARE, MAX_VALUE_FOR_SQUARE);
        let corner4 = Point2::new(MAX_VALUE_FOR_SQUARE, MAX_VALUE_FOR_SQUARE);
        pc.push_line(corner1, corner2).unwrap();
        pc.push_line(corner2, corner4).unwrap();
        pc.push_line(corner4, corner3).unwrap();
        pc.push_line(corner3, corner1).unwrap();
        pc
    }

    #[test]
    fn draw_composite_lines() {
        let pc = ParametricComposite::new();
        let pc = add_letter_a_to_pc(pc);
        let canvas = draw::CanvasPDE2D::new("outputs/tests/test_comp_lines.bmp", [1000, 500]);
        canvas
            .draw_parametric(&pc, 10, full_palette::AMBER)
            .unwrap();
    }
    #[test]
    fn draw_result_with_geom() {
        let pc = ParametricComposite::new();
        //let pc = add_letter_a_to_pc(pc);
        let pc = add_border_to_pc(pc);
        pc.print_locations();
        let resolution: [usize; 2] = [100, 100];
        let canvas = draw::CanvasPDE2D::new("outputs/tests/test_result_w_geom.bmp", resolution);
        let renderer = MonteCarloPDE2D::new(resolution, 25, 100, 2f32.powi(-10));
        let result = renderer
            .find_pde(
                &pc,
                Method2D::Laplacian(|p| p.x + p.y + 2000000f32 / (1000f32 + p.x) * (500f32 + p.y)),
            )
            .unwrap();
        canvas.draw_result(&result).unwrap();
        //canvas.draw_parametric(&pc, 1, full_palette::BLACK).unwrap();
        canvas.present().unwrap();
    }

    #[test]
    fn line_queries() {
        let pc = ParametricComposite::new();
        let pc = add_letter_a_to_pc(pc);
        let query: [Point2<f32>; 2] = [Point2::new(0.5f32, 0.4f32), Point2::new(0.7f32, 10f32)];
        //println!(
        //    "Closest point is at x: {}, y : {}",
        //    pc.closest_point(query[0]).expect("").x,
        //    pc.closest_point(query[0]).expect("").y
        //);
        assert_eq!(pc.closest_point(query[0]).expect("").x, 0.5f32);
        assert_eq!(pc.closest_point(query[0]).expect("").y, 0.5f32);
        assert_eq!(pc.closest_point(query[1]).expect("").x, 0.5f32);
        assert_eq!(pc.closest_point(query[1]).expect("").y, 1f32);
    }
}
