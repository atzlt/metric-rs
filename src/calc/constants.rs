use crate::objects::Point;

pub const EPSILON: f64 = 1e-10;
pub const DEG: f64 = std::f64::consts::PI / 180.0;
pub const ROUND: f64 = 2.0 * std::f64::consts::PI;

pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
