use crate::geometry::ClosestPointQuery;

pub enum Method2D {
    Laplacian,
}

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
    pub fn find_pde<G>(&self, geometry: &G, method: Method2D) -> Vec<f32>
    where
        G: ClosestPointQuery,
    {
        match method {
            Method2D::Laplacian => {
                vec![0f32; self.resolution[0] * self.resolution[1]]
            }
        }
    }
}
