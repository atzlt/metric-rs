use crate::objects::{Circle, Point};

/// Construct a point `A` on circle, by the angle `AOx`.
pub fn on_circle(c: Circle, angle: f64) -> Point {
    Point {
        x: c.o.x + c.r * angle.cos(),
        y: c.o.y + c.r * angle.sin(),
    }
}

/// Construct a point `P` on a segment, by `vec(PA) / vec(PB) = r`.
pub fn on_segment((a, b): (Point, Point), r: f64) -> Point {
    a * (1.0 - r) + b * r
}
