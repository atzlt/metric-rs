use crate::objects::{Circle, Line, Point};

use super::{
    basic::{is_parallel, DistanceTo, Intersect},
    constructs::{midpoint, perp, projection},
};

/// A trait for reflection (in `T`). Provides `reflect_in` function.
pub trait ReflectIn<T> {
    /// Reflect in `T`.
    fn reflect_in(self, obj: T) -> Self;
}

impl ReflectIn<Point> for Point {
    /// Reflect Point in Point.
    #[inline]
    fn reflect_in(self, obj: Point) -> Self {
        obj * 2.0 - self
    }
}

impl ReflectIn<Line> for Point {
    /// Reflect Point in Line.
    fn reflect_in(self, obj: Line) -> Self {
        let Line { a, b, c } = obj;
        let m = b * b + a * a;
        let n = b * b - a * a;
        Point {
            x: (self.x * n - 2.0 * a * (b * self.y + c)) / m,
            y: (-self.y * n - 2.0 * b * (a * self.x + c)) / m,
        }
    }
}

impl ReflectIn<Point> for Line {
    /// Reflect Line in Point.
    #[inline]
    fn reflect_in(self, obj: Point) -> Self {
        Line {
            a: self.a,
            b: self.b,
            c: -self.c - 2.0 * (self.a * obj.x + self.b * obj.y),
        }
    }
}

impl ReflectIn<Line> for Line {
    /// Reflect Line in Line.
    fn reflect_in(self, obj: Line) -> Self {
        if is_parallel(self, obj) {
            Line {
                a: self.a,
                b: self.b,
                c: 2.0 * obj.c - self.c,
            }
        } else {
            let (a, b) = (obj.a, obj.b);
            let (c, d) = (self.a, self.b);
            let a0 = a * a * c + 2.0 * a * b * d - b * b * c;
            let b0 = 2.0 * a * b * c + (b * b - a * a) * d;
            Line::from_slope_and_point(a0, b0, self.inter(obj).unwrap())
        }
    }
}

impl ReflectIn<Point> for Circle {
    /// Reflect Circle in Point.
    #[inline]
    fn reflect_in(self, obj: Point) -> Self {
        Circle {
            o: self.o.reflect_in(obj),
            r: self.r,
        }
    }
}

impl ReflectIn<Line> for Circle {
    /// Reflect Circle in Line.
    #[inline]
    fn reflect_in(self, obj: Line) -> Self {
        Circle {
            o: self.o.reflect_in(obj),
            r: self.r,
        }
    }
}

/// A trait for (circular) inversion. Provides `invert_in` function.
pub trait Invertible {
    /// The type of the inverted shape.
    type Inverted;
    /// Invert with center `o` and radius `r` (can be negative).
    fn invert_in(self, o: Point, p: f64) -> Self::Inverted;
}

/// The possible results of an inversion of a Line (or a Circle; they're the same).
#[derive(Debug, PartialEq)]
pub enum LineInvertResult {
    Line(Line),
    Circle(Circle),
}

/// Possibly you're inverting the center itself. This is the error enum.
#[derive(Debug, PartialEq, Eq)]
pub enum PointInversionErr {
    OverlapWithCenter,
}

impl Invertible for Point {
    type Inverted = Result<Point, PointInversionErr>;
    /// Invert a Point.
    fn invert_in(self, o: Point, p: f64) -> Self::Inverted {
        if self == o {
            Err(PointInversionErr::OverlapWithCenter)
        } else {
            let d = self - o;
            let scale = p / self.distance_sq(o);
            Ok(o + d * scale)
        }
    }
}

impl Invertible for Line {
    type Inverted = LineInvertResult;
    /// Invert a Line.
    fn invert_in(self, o: Point, p: f64) -> Self::Inverted {
        if self.is_through(o) {
            LineInvertResult::Line(self)
        } else {
            let t = projection(o, self);
            let o1 = midpoint(t.invert_in(o, p).unwrap(), o);
            LineInvertResult::Circle(Circle::from_center_point(o1, o).unwrap())
        }
    }
}

impl Invertible for Circle {
    type Inverted = LineInvertResult;
    /// Invert a Circle.
    fn invert_in(self, o: Point, p: f64) -> Self::Inverted {
        if self.is_through(o) {
            let m = midpoint(self.o.invert_in(o, p).unwrap(), o);
            LineInvertResult::Line(perp(m, Line::from_2p(m, o).unwrap()))
        } else {
            let (a, b) = self.inter(Line::from_2p(o, self.o).unwrap());
            let a = a.unwrap();
            let b = b.unwrap();
            let a0 = a.invert_in(o, p).unwrap();
            let b0 = b.invert_in(o, p).unwrap();
            let o1 = midpoint(a0, b0);
            LineInvertResult::Circle(Circle::from_center_point(o1, a0).unwrap())
        }
    }
}

/// A trait for rotation. Provides `rotate` function.
/// **This function should rotate counterclockwise by default.**
pub trait Rotatable {
    /// Rotate by angle.
    fn rotate(self, o: Point, angle: f64) -> Self;
}

impl Rotatable for Point {
    /// Rotate a Point around a Point by angle.
    fn rotate(self, o: Point, angle: f64) -> Self {
        let dx = self.x - o.x;
        let dy = self.y - o.y;
        let s = angle.sin();
        let c = angle.cos();
        Point {
            x: dx * c - dy * s + o.x,
            y: dy * c + dx * s + o.y,
        }
    }
}

impl Rotatable for Line {
    /// Rotate a Line around a Point by angle.
    fn rotate(self, o: Point, angle: f64) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();
        let Line { a, b, c } = self;
        let a0 = a * cos - b * sin;
        let b0 = b * cos + a * sin;
        let c0 = a * o.x + b * o.y + c;
        Line {
            a: a0,
            b: b0,
            c: c0,
        }
    }
}
