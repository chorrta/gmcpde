use anyhow::{Context, Result, bail};
use cgmath::{Point2, Vector2, dot, prelude::*};

#[derive(Clone)]
pub struct ParametricComposite {
    components: Vec<Geom2D>,
}

#[derive(Clone)]
pub struct Line {
    point1: Point2<f32>,
    point2: Point2<f32>,
}
#[derive(Clone)]
pub struct Bezier {
    point1: Point2<f32>,
    point2: Point2<f32>,
    point3: Point2<f32>,
    point4: Point2<f32>,
}
#[derive(Clone)]
pub enum Geom2D {
    Line(Line),
    //    Bezier(Bezier),
}

impl Line {
    pub fn new(start_point: Point2<f32>, end_point: Point2<f32>) -> Result<Self> {
        if start_point != end_point {
            Ok(Self {
                point1: start_point,
                point2: end_point,
            })
        } else {
            bail!("Startpoint cannot be the same as the end point.")
        }
    }
    pub fn points(&self) -> [Point2<f32>; 2] {
        [self.point1, self.point2]
    }
}

impl ParametricComposite {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn push_line(&mut self, start_point: Point2<f32>, end_point: Point2<f32>) -> Result<()> {
        let new_line = Geom2D::Line(
            Line::new(start_point, end_point).context("Failed to push new line to composite.")?,
        );
        Ok(self.components.push(new_line))
    }
}

impl Parametric for Line {
    fn point_along(&self, t: f32) -> Point2<f32> {
        self.point1 + (self.point2 - self.point1) * t.clamp(0f32, 1f32)
    }
}

pub trait Parametric {
    fn point_along(&self, t: f32) -> Point2<f32>;
}

impl<'a> IntoIterator for &'a ParametricComposite {
    type Item = &'a Geom2D;
    type IntoIter = core::slice::Iter<'a, Geom2D>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter()
    }
}

pub trait ClosestPointQuery {
    fn closest_point(&self, query: Point2<f32>) -> Option<Point2<f32>>;
}

impl ClosestPointQuery for Line {
    fn closest_point(&self, query: Point2<f32>) -> Option<Point2<f32>> {
        let t: f32 = dot(query - self.point1, self.point2 - self.point1)
            / self.point1.distance2(self.point2);
        let closest_point: Point2<f32> = self.point_along(t);
        Some(closest_point)
    }
}

impl ClosestPointQuery for Geom2D {
    fn closest_point(&self, query: Point2<f32>) -> Option<Point2<f32>> {
        match self {
            Geom2D::Line(line) => line.closest_point(query),
        }
    }
}

impl ClosestPointQuery for ParametricComposite {
    fn closest_point(&self, query: Point2<f32>) -> Option<Point2<f32>> {
        self.into_iter()
            .filter_map(|c| c.closest_point(query))
            .min_by(|x, y| {
                query
                    .distance2(*x)
                    .partial_cmp(&query.distance2(*y))
                    .expect(
                        "Closest point can not be found, because there are NaN values in one of the points."
                    )
            })
    }
}
