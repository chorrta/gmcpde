pub mod draw {
    use crate::geometry;
    use anyhow::{Context, Result, bail};
    use cgmath::Point2;
    use plotters::{
        coord::types::RangedCoordf32,
        prelude::*,
        style::colors::colormaps,
        style::full_palette::{AMBER, BLUEGREY_A700, DEEPORANGE_A700},
    };

    pub struct CanvasPDE2D<'a> {
        resolution: [usize; 2],
        root: DrawingArea<BitMapBackend<'a>, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    }
    impl<'a> CanvasPDE2D<'a> {
        pub fn new(file_path: &'a str, resolution: [usize; 2]) -> Self {
            let backend =
                BitMapBackend::new(file_path, (resolution[0] as u32, resolution[1] as u32));
            let root = backend.into_drawing_area().apply_coord_spec(Cartesian2d::<
                RangedCoordf32,
                RangedCoordf32,
            >::new(
                0f32..1f32,
                1f32..0f32,
                (0..resolution[0] as i32, 0..resolution[1] as i32),
            ));
            Self { resolution, root }
        }
        pub fn present(self) -> Result<()> {
            self.root.present()?;
            Ok(())
        }
        pub fn draw_parametric(
            &self,
            pc: &geometry::ParametricComposite,
            line_width: u32,
            line_colour: RGBColor,
        ) -> Result<()> {
            for component in pc.into_iter() {
                match component {
                    geometry::Geom2D::Line(line) => self.root.draw(&PathElement::new(
                        [translate(&line.points()[0]), translate(&line.points()[1])],
                        Into::<ShapeStyle>::into(&line_colour)
                            .filled()
                            .stroke_width(line_width),
                    ))?,
                };
            }
            Ok(())
        }
        pub fn draw_result(&self, res_to_draw: &Vec<f32>) -> Result<()> {
            if res_to_draw.len() != self.resolution[0] * self.resolution[1] {
                bail!("Result does not match canvas resolution.")
            }
            let colormap = DerivedColorMap::new(&[BLUEGREY_A700, AMBER, DEEPORANGE_A700]);
            let min: &f32 = res_to_draw
                .into_iter()
                .reduce(|l, r| if l >= r { r } else { l })
                .expect("Failed to find minimum in result vector.");
            let max: &f32 = res_to_draw
                .into_iter()
                .reduce(|l, r| if l <= r { r } else { l })
                .expect("Failed to find maximum in result vector.");
            let max: &f32 = if min != max { max } else { &(max + 1f32) };
            for value in res_to_draw.iter().enumerate() {
                let x_coord = value.0 % self.resolution[0];
                // Here the division is floor division because we are using rust integers.
                let y_coord = value.0 / self.resolution[0];
                self.root.draw_pixel(
                    (
                        x_coord as f32 / self.resolution[0] as f32,
                        y_coord as f32 / self.resolution[1] as f32,
                    ),
                    &colormap.get_color_normalized(value.1.clone(), min.clone(), max.clone()),
                )?;
            }
            Ok(())
        }
    }
    fn translate(from: &Point2<f32>) -> (f32, f32) {
        (from.x.clone() as f32, from.y.clone() as f32)
    }
}
