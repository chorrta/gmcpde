use crate::geometry::ClosestPointQuery;
use anyhow::{Context, Result, bail};
use cgmath::{Point2, Rad, Vector2, prelude::*};
use pollster;
use rand::distr::StandardUniform;
use rand::prelude::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use rayon::prelude;
use std::sync::Arc;
use wgpu;

pub enum Method2D {
    Laplacian(fn(Point2<f32>) -> f32),
}
enum LaplaceResult {}

pub struct MonteCarloPDE2D {
    pub resolution: [usize; 2],
    pub num_walks_pixel: usize,
    pub max_walk_length: usize,
    pub stop_tol: f32,
}

impl MonteCarloPDE2D {
    pub fn new(
        resolution: [usize; 2],
        num_walks_pixel: usize,
        max_walk_length: usize,
        stop_tol: f32,
    ) -> Self {
        Self {
            resolution,
            num_walks_pixel,
            max_walk_length,
            stop_tol,
        }
    }
    pub fn find_pde<G>(&self, geometry: &G, method: Method2D) -> Result<Vec<f32>>
    where
        G: ClosestPointQuery + Clone + Send + Sync,
    {
        // let instance = wgpu::Instance::new(&Default::default());
        // let adapter = instance.request_adapter(&Default::default()).await?;
        // let (device, queue) = adapter.request_device(&Default::default()).await?;

        match method {
            Method2D::Laplacian(boundry_function) => {
                // let shader =
                //     device.create_shader_module(wgpu::include_wgsl!("compute/laplacian.wgsl"));

                // let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                //     label: Some("Montecarlo 2D"),
                //     layout: None,
                //     module: &shader,
                //     entry_point: None,
                //     compilation_options: Default::default(),
                //     cache: Default::default(),
                // });
                let result_buffer = vec![0f32; self.resolution[0] * self.resolution[1]];
                let pid_to_point = |pixel_id| {
                    let x_coord: usize = pixel_id % self.resolution[0];
                    let y_coord: usize = pixel_id / self.resolution[0];
                    Point2::new(
                        x_coord as f32 / self.resolution[0] as f32,
                        y_coord as f32 / self.resolution[1] as f32,
                    )
                };
                let result_buffer: Vec<f32> = result_buffer
                    .into_par_iter()
                    .enumerate()
                    .map(|pixel| {
                        self.wos_laplace(geometry, pid_to_point(pixel.0), &boundry_function)
                            .context("When using WoS Laplace.")
                            .unwrap()
                    })
                    .collect();
                Ok(result_buffer)
            }
        }
    }
    fn wos_laplace<G>(
        &self,
        geometry: &G,
        point_to_check_initial: Point2<f32>,
        boundry_function: &fn(Point2<f32>) -> f32,
    ) -> Result<f32>
    where
        G: ClosestPointQuery,
    {
        let mut result_of_pixel: f32 = 0f32;
        let mut rng = rand::rng();
        for walk in 0..self.num_walks_pixel {
            let mut point_to_check = point_to_check_initial.clone();
            let mut n = 0;
            let mut previous_cbp = Point2::new(0f32, 0f32);
            let mut previous_point_to_check = point_to_check.clone();
            while n < self.max_walk_length {
                let Some(closest_boundry_point) = geometry.closest_point(point_to_check) else {
                    bail!(
                        "Couldn't find closest boundry point.
                            Step {n}, Walk {walk},
                            Query point is ({}, {}), initial point is ({}, {}),
                            prev point({},{}), prev cbp ({}, {})",
                        point_to_check.x,
                        point_to_check.y,
                        point_to_check_initial.x,
                        point_to_check_initial.y,
                        previous_point_to_check.x,
                        previous_point_to_check.y,
                        previous_cbp.x,
                        previous_cbp.y
                    );
                };
                let sphere_radius: f32 = closest_boundry_point.distance(point_to_check);
                if sphere_radius < self.stop_tol {
                    break;
                }
                let random_direction: Rad<f32> = Rad(2.0
                    * std::f32::consts::PI
                    * rng.sample::<f32, StandardUniform>(StandardUniform));
                point_to_check = point_to_check
                    + Vector2::new(
                        sphere_radius * random_direction.cos(),
                        sphere_radius * random_direction.sin(),
                    );
                n += 1;
                #[cfg(test)]
                if point_to_check_initial == Point2::new(2f32, 2f32) && walk == 0 {
                    println!(
                        "Step {n} point to check is ({}, {})",
                        point_to_check.x, point_to_check.y
                    );
                    println!(
                        "  Sphere radius is {}, Stopping tolerence is {})",
                        sphere_radius, self.stop_tol
                    );
                }
                previous_cbp = closest_boundry_point;
                previous_point_to_check = point_to_check.clone();
            }
            result_of_pixel += boundry_function(point_to_check);
        }
        Ok(result_of_pixel / (self.num_walks_pixel as f32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::*;
    use crate::ui::draw;

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
    fn make_test_scenario_geom() -> ParametricComposite {
        let pc = ParametricComposite::new();
        let pc = add_border_to_pc(pc);
        pc
    }
    #[test]
    fn print_walk() -> Result<()> {
        let pc = make_test_scenario_geom();
        let resolution: [usize; 2] = [50, 50];
        let renderer = MonteCarloPDE2D::new(resolution, 20, 100, 2f32.powi(-10));
        let _ = renderer.find_pde(&pc, Method2D::Laplacian(|_p| 1f32))?;
        Ok(())
    }
}
