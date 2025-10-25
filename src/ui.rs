pub mod draw {
    use crate::geometry;
    use anyhow::{Context, Result};
    use cgmath::Point2;
    use plotters::{coord::types::RangedCoordf32, prelude::*};

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
        let root = SVGBackend::new("output.svg", (1000, 1000)).into_drawing_area();
        let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
            0f32..1f32,
            1f32..0f32,
            (0..1000, 0..1000),
        ));
        root.fill(&BLUE)?;
        for component in pc.into_iter() {
            match component {
                geometry::Geom2D::Line(line) => root.draw(&PathElement::new(
                    [translate(&line.points()[0]), translate(&line.points()[1])],
                    Into::<ShapeStyle>::into(&WHITE).filled().stroke_width(10),
                ))?,
            };
        }
        root.present()?;
        Ok(())
    }

    fn translate(from: &Point2<f32>) -> (f32, f32) {
        (from.x.clone() as f32, from.y.clone() as f32)
    }
}
