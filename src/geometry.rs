use anyhow::{Context, Result, bail};
use cgmath::{Point2, prelude::*};

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
    Bezier(Bezier),
}

// pub struct ClampedF32(f32);
//
// impl ClampedF32 {
//     pub fn new(value: f32) -> Result<Self> {
//         if (0f32..=1f32).contains(&value) {
//             Ok(Self(value))
//         } else {
//             bail!("Value must be in range [0; 1]")
//         }
//     }
//     pub fn value(&self) -> f32 {
//         self.0
//     }
// }

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
