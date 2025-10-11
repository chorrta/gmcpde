use anyhow::{Context, Result, bail};
use cgmath::{Point2, prelude::*};

pub struct ParametricComposite {
    lines: Vec<Line>,
    beziers: Vec<Bezier>,
}

pub struct Line {
    point1: Point2<f32>,
    point2: Point2<f32>,
}

struct Bezier {
    point1: Point2<f32>,
    point2: Point2<f32>,
    point3: Point2<f32>,
    point4: Point2<f32>,
}

pub struct ClampedF32(f32);

impl ClampedF32 {
    pub fn new(value: f32) -> Result<Self> {
        if (0f32..=1f32).contains(&value) {
            Ok(Self(value))
        } else {
            bail!("Value must be in range [0; 1]")
        }
    }
    pub fn value(&self) -> f32 {
        self.0
    }
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

impl Parametric for Line {
    fn point_along(&self, t: ClampedF32) -> Point2<f32> {
        self.point1 + (self.point2 - self.point1) * t.value()
    }
}

pub trait Parametric {
    fn point_along(&self, t: ClampedF32) -> Point2<f32>;
}
