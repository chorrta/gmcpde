pub mod draw {
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
}
