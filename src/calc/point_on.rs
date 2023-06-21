use crate::objects::{Circle, Point};

/// Trait for constructing a point on another object by a parameter `pos` controlling position.
pub trait PointOn {
    /// Construct a point on `self` by a position given by `pos`.
    fn point_on(&self, pos: f64) -> Point;
}

impl PointOn for Circle {
    /// Construct a point `A` on circle, by the angle `AOx`.
    #[inline]
    fn point_on(&self, angle: f64) -> Point {
        Point {
            x: self.O.x + self.r * angle.cos(),
            y: self.O.y + self.r * angle.sin(),
        }
    }
}
