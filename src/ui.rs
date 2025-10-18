pub mod draw {
    use crate::geometry;
    use anyhow::{Context, Result};
    use cgmath::Point2;
    use plotters::prelude::*;

    pub fn draw_circle() -> Result<(), Box<dyn std::error::Error>> {
        let backend = SVGBackend::new("output.svg", (800, 600)).into_drawing_area();
        backend.fill(&WHITE)?;
        backend.draw(&Circle::new(
            (400, 300),
            200,
            Into::<ShapeStyle>::into(&RED).filled(),
        ))?;
        backend.present()?;
        Ok(())
    }

    pub fn draw_parametric(pc: &geometry::ParametricComposite) -> Result<()> {
        let backend = SVGBackend::new("output.svg", (100, 100)).into_drawing_area();
        backend.fill(&BLUE)?;
        for component in pc.into_iter() {
            match component {
                geometry::Geom2D::Line(line) => backend.draw(&PathElement::new(
                    [translate(&line.points()[0]), translate(&line.points()[1])],
                    Into::<ShapeStyle>::into(&WHITE).filled().stroke_width(10),
                ))?,
                geometry::Geom2D::Bezier(bezier) => panic!("Bezier drawing not implemented yet."),
            };
        }
        backend.present()?;
        Ok(())
    }

    fn translate(from: &Point2<f32>) -> (i32, i32) {
        (from.x.clone() as i32, from.y.clone() as i32)
    }
}
