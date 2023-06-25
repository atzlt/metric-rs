#![allow(non_snake_case)]

use crate::objects::{Circle, Line, Point};

use super::{
    basic::{is_parallel, Distance, Intersect, TestThrough},
    construct::{midpoint, perp, projection},
    exception::{CalcException, Result},
};

/// A trait for reflection (in `T`). Provides `reflect_in` function.
pub trait Reflect<T> {
    /// Reflect in `T`.
    fn reflect_in(self, obj: T) -> Self;
}

impl Reflect<Point> for Point {
    /// Reflect Point in Point.
    #[inline]
    fn reflect_in(self, P: Point) -> Self {
        P * 2.0 - self
    }
}

impl Reflect<Line> for Point {
    /// Reflect Point in Line.
    fn reflect_in(self, l: Line) -> Self {
        let Line { a, b, c } = l;
        let m = b * b + a * a;
        let n = b * b - a * a;
        Point {
            x: (self.x * n - 2.0 * a * (b * self.y + c)) / m,
            y: (-self.y * n - 2.0 * b * (a * self.x + c)) / m,
        }
    }
}

impl Reflect<Point> for Line {
    /// Reflect Line in Point.
    #[inline]
    fn reflect_in(self, P: Point) -> Self {
        Line {
            a: self.a,
            b: self.b,
            c: -self.c - 2.0 * (self.a * P.x + self.b * P.y),
        }
    }
}

impl Reflect<Line> for Line {
    /// Reflect Line in Line.
    fn reflect_in(self, l: Line) -> Self {
        if is_parallel(self, l) {
            Line {
                a: self.a,
                b: self.b,
                c: 2.0 * l.c - self.c,
            }
        } else {
            let (a, b) = (l.a, l.b);
            let (c, d) = (self.a, self.b);
            let a0 = a * a * c + 2.0 * a * b * d - b * b * c;
            let b0 = 2.0 * a * b * c + (b * b - a * a) * d;
            Line::from_slope_and_point(a0, b0, self.inter(l).unwrap())
        }
    }
}

impl Reflect<Point> for Circle {
    /// Reflect Circle in Point.
    #[inline]
    fn reflect_in(self, P: Point) -> Self {
        Circle {
            O: self.O.reflect_in(P),
            r: self.r,
        }
    }
}

impl Reflect<Line> for Circle {
    /// Reflect Circle in Line.
    #[inline]
    fn reflect_in(self, l: Line) -> Self {
        Circle {
            O: self.O.reflect_in(l),
            r: self.r,
        }
    }
}

/// A trait for (circular) inversion. Provides `invert_in` function.
pub trait Invert {
    /// The type of the inverted shape.
    type Inverted;
    /// Invert with center `o` and radius `r` (can be negative).
    fn invert_in(self, O: Point, p: f64) -> Self::Inverted;
}

/// The possible results of an inversion of a Line (or a Circle; they're the same).
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, PartialEq)]
pub enum LineInverted {
    Line(Line),
    Circle(Circle),
}

impl Invert for Point {
    type Inverted = Result<Point>;
    /// Invert a Point.
    fn invert_in(self, O: Point, p: f64) -> Self::Inverted {
        if self == O {
            Err(CalcException::OverlappingPoint)
        } else {
            let d = self - O;
            let scale = p / self.distance_sq(O);
            Ok(O + d * scale)
        }
    }
}

impl Invert for Line {
    type Inverted = LineInverted;
    /// Invert a Line.
    fn invert_in(self, O: Point, p: f64) -> Self::Inverted {
        if self.is_through(O) {
            LineInverted::Line(self)
        } else {
            let t = projection(O, self);
            let o1 = midpoint(t.invert_in(O, p).unwrap(), O);
            LineInverted::Circle(Circle::from_center_point(o1, O).unwrap())
        }
    }
}

impl Invert for Circle {
    type Inverted = LineInverted;
    /// Invert a Circle.
    fn invert_in(self, O: Point, p: f64) -> Self::Inverted {
        if self.is_through(O) {
            let m = midpoint(self.O.invert_in(O, p).unwrap(), O);
            LineInverted::Line(perp(m, Line::from_2p(m, O).unwrap()))
        } else {
            let (a, b) = self.inter(Line::from_2p(O, self.O).unwrap()).unwrap();
            let a0 = a.invert_in(O, p).unwrap();
            let b0 = b.invert_in(O, p).unwrap();
            let o1 = midpoint(a0, b0);
            LineInverted::Circle(Circle::from_center_point(o1, a0).unwrap())
        }
    }
}

/// A trait for rotation. Provides `rotate` function.
/// **This function should rotate counterclockwise by default**, because otherwise it would
/// be confusing.
pub trait Rotate {
    /// Rotate by angle.
    fn rotate(self, O: Point, angle: f64) -> Self;
}

impl Rotate for Point {
    /// Rotate a Point around a Point by angle.
    fn rotate(self, O: Point, angle: f64) -> Self {
        let dx = self.x - O.x;
        let dy = self.y - O.y;
        let s = angle.sin();
        let c = angle.cos();
        Point {
            x: dx * c - dy * s + O.x,
            y: dy * c + dx * s + O.y,
        }
    }
}

impl Rotate for Line {
    /// Rotate a Line around a Point by angle.
    fn rotate(self, O: Point, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        let Line { a, b, c } = self;
        let a0 = a * cos - b * sin;
        let b0 = b * cos + a * sin;
        let c0 = a * O.x + b * O.y + c;
        Line {
            a: a0,
            b: b0,
            c: c0,
        }
    }
}

impl Rotate for Circle {
    /// Rotate a Circle around a Point by angle.
    #[inline]
    fn rotate(self, O: Point, angle: f64) -> Self {
        Circle {
            O: self.O.rotate(O, angle),
            r: self.r,
        }
    }
}

/// A trait for scaling.
pub trait Scale {
    /// Scale an object with center `O` and ratio `r`.
    fn scale(self, O: Point, r: f64) -> Self;
}

impl Scale for Point {
    #[inline]
    fn scale(self, O: Point, r: f64) -> Self {
        self * r - O * (r - 1.0)
    }
}

impl Scale for Line {
    #[inline]
    fn scale(self, O: Point, r: f64) -> Self {
        Line {
            a: self.a,
            b: self.b,
            c: (self.a * O.x + self.b * O.y) * (r - 1.0) + self.c * r,
        }
    }
}

impl Scale for Circle {
    #[inline]
    fn scale(self, center: Point, ratio: f64) -> Self {
        Circle {
            O: self.O.scale(center, ratio),
            r: self.r * ratio,
        }
    }
}
