#![allow(non_snake_case)]

use crate::objects::{Circle, Line, Point};

use super::{
    constants::EPSILON,
    construct::perp_bisect,
    exception::{CalcException, Result},
};

/// Test if two floats are _almost_ equal.
#[inline]
fn aprx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

/// Test if two lines are parallel.
#[inline]
pub fn is_parallel(l: Line, k: Line) -> bool {
    aprx_eq(l.a * k.b, l.b * k.a)
}

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Line {
    /// Construct new Line from coefficients: `ax + by + c = 0`.
    /// `a` and `b` cannot be both zero.
    #[inline]
    pub fn from_coeff(a: f64, b: f64, c: f64) -> Result<Line> {
        if a == 0.0 && b == 0.0 {
            return Err(CalcException::ZeroCoefficient);
        }
        Ok(Line { a, b, c })
    }
    /// Construct new Line from two coefficients `ax + by + ?? = 0` and
    /// a given point that the line passes.
    #[inline]
    pub fn from_slope_and_point(a: f64, b: f64, P: Point) -> Self {
        Line {
            a,
            b,
            c: -a * P.x - b * P.y,
        }
    }
    /// Construct new Line passing through two Points.
    /// If the two Points overlap return `OverlappingPoint` error.
    #[inline]
    pub fn from_2p(A: Point, B: Point) -> Result<Self> {
        if A == B {
            Err(CalcException::OverlappingPoint)
        } else {
            Ok(Line {
                a: A.y - B.y,
                b: B.x - A.x,
                c: A.x * B.y - A.y * B.x,
            })
        }
    }
}

impl Circle {
    /// Construct a Circle with center `o` and radius `r`.
    /// If the radius given is nonpositive return `NonpositiveRadius` error.
    #[inline]
    pub fn from_center_radius(O: Point, R: f64) -> Result<Self> {
        if R <= 0.0 {
            Err(CalcException::NonpositiveRadius)
        } else {
            Ok(Circle { O, r: R })
        }
    }
    /// Construct a Circle with center `o` and a point it passes through `a`.
    #[inline]
    pub fn from_center_point(O: Point, A: Point) -> Result<Self> {
        if O == A {
            Err(CalcException::OverlappingPoint)
        } else {
            Ok(Circle {
                O,
                r: O.distance(A),
            })
        }
    }
    /// Construct a Circle passing through three Points.
    /// If any two of them overlap return `OverlappingPoint` error.
    pub fn from_3p(A: Point, B: Point, C: Point) -> Result<Self> {
        let O = perp_bisect(A, B)?.inter(perp_bisect(B, C)?)?;
        let r = O.distance(A);
        Ok(Circle { O, r })
    }
}

/// A trait for computing distance.
/// The `distance_sq` function _must_ be implemented. The `distance` function is computed
/// using `distance_sq`, so its implementation is hence optional.
pub trait Distance<T>
where
    Self: Sized,
{
    /// The square of the distance.
    fn distance_sq(self, obj: T) -> f64;

    /// The distance
    #[inline]
    fn distance(self, obj: T) -> f64 {
        self.distance_sq(obj).sqrt()
    }
}

impl Distance<Point> for Point {
    fn distance_sq(self, P: Point) -> f64 {
        let dx = self.x - P.x;
        let dy = self.y - P.y;
        dx * dx + dy * dy
    }
}

impl Distance<Line> for Point {
    fn distance_sq(self, l: Line) -> f64 {
        let z = self.x * l.a + self.y * l.b + l.c;
        z * z / (l.a * l.a + l.b * l.b)
    }
}

impl Distance<Line> for Line {
    fn distance_sq(self, l: Line) -> f64 {
        if !is_parallel(self, l) {
            0.0
        } else {
            let z = self.c - l.c;
            z * z / (self.a * self.a + self.b * self.b)
        }
    }
}

/// The angle defined by three points, the one in `[0, pi / 2]`.
pub fn angle(A: Point, O: Point, B: Point) -> Result<f64> {
    if A == O || B == O {
        return Err(CalcException::OverlappingPoint);
    }
    let (dx1, dy1, dx2, dy2) = (A.y - O.y, O.x - A.x, B.y - O.y, O.x - B.x);
    let a = dx1 * dx1 + dy1 * dy1;
    let b = dx2 * dx2 + dy2 * dy2;
    let p = (dx1 * dx2 + dy1 * dy2) / (a * b).sqrt();
    Ok(p.acos())
}

/// The angle between two lines, the one in `[0, pi / 2]`.
pub fn angle_between(l: Line, k: Line) -> f64 {
    let (a, b) = (l.a, l.b);
    let (c, d) = (k.a, k.b);
    let a0 = a * a + b * b;
    let b0 = c * c + d * d;
    let p = (a * c + b * d) / (a0 * b0).sqrt();
    p.abs().acos()
}

impl std::cmp::PartialEq for Point {
    /// If two Points are _approximately_ equal.
    /// We say _approximately_ because there could be error.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        aprx_eq(self.x, other.x) && aprx_eq(self.y, other.y)
    }
}

impl std::cmp::PartialEq for Line {
    /// If two Lines _approximately_ overlaps.
    /// We say _approximately_ because there could be error.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        is_parallel(*self, *other) && aprx_eq(self.c, other.c)
    }
}

impl std::cmp::PartialEq for Circle {
    /// If two Circles _approximately_ overlaps.
    /// We say _approximately_ because there could be error.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.O == other.O && aprx_eq(self.r, other.r)
    }
}

/// A trait for constructing intersections.
pub trait Intersect<T> {
    /// The result of intersection.
    /// This is generally `Option<Point>`, or a tuple of `Option<Point>`.
    /// Use `Option` because there might be no intersections.
    type InterResult;
    /// Intersection.
    fn inter(self, obj: T) -> Result<Self::InterResult>;
    /// Intersection _with a common point given_.
    /// This can simplify calculation (using Vieta's theorem).
    /// **Do notice that** this common point will _directly_ affect the result.
    /// If a wrong common point is given the result will be _totally wrong_.
    /// The given common point is always the last element of the tuple.
    fn inter_common(self, _: T, common: Point) -> Result<Self::InterResult>;
}

impl Intersect<Line> for Line {
    type InterResult = Point;
    fn inter(self, obj: Line) -> Result<Self::InterResult> {
        if is_parallel(self, obj) {
            Err(CalcException::NoIntersection)
        } else {
            let a = self.b * obj.c - obj.b * self.c;
            let b = self.c * obj.a - obj.c * self.a;
            let d = self.a * obj.b - obj.a * self.b;
            Ok(Point { x: a / d, y: b / d })
        }
    }
    #[inline]
    fn inter_common(self, _: Line, common: Point) -> Result<Self::InterResult> {
        Ok(common)
    }
}

impl Intersect<Circle> for Line {
    type InterResult = (Point, Point);
    fn inter(self, obj: Circle) -> Result<Self::InterResult> {
        let Circle { O, r } = obj;
        let Line { a, b, c } = self;
        if a != 0.0 {
            let ya = a * a + b * b;
            let yb = 2.0 * ((a * O.x + c) * b - a * a * O.y);
            let yc = a * a * (O.y * O.y - r * r) + (a * O.x + c) * (a * O.x + c);
            let disc = yb * yb - 4.0 * ya * yc;
            if disc < 0.0 {
                return Err(CalcException::NoIntersection);
            }
            let disc = disc.sqrt();
            let y1 = (-yb + disc) / ya / 2.0;
            let y2 = (-yb - disc) / ya / 2.0;
            Ok((
                Point {
                    x: -(b * y1 + c) / a,
                    y: y1,
                },
                Point {
                    x: -(b * y2 + c) / a,
                    y: y2,
                },
            ))
        } else {
            let xa = b * b;
            let xb = -2.0 * b * b * O.x;
            let xc = b * b * (O.x * O.x - r * r) + (b * O.x + c) * (b * O.x + c);
            let disc = xb * xb - 4.0 * xa * xc;
            if disc < 0.0 {
                return Err(CalcException::NoIntersection);
            }
            let disc = disc.sqrt();
            let x1 = (-xb + disc) / xa / 2.0;
            let x2 = (-xb - disc) / xa / 2.0;
            Ok((Point { x: x1, y: -c / b }, Point { x: x2, y: -c / b }))
        }
    }
    fn inter_common(self, obj: Circle, common: Point) -> Result<Self::InterResult> {
        let O = obj.O;
        let Line { a, b, c } = self;
        if a != 0.0 {
            let ya = a * a + b * b;
            let yb = 2.0 * ((a * O.x + c) * b - a * a * O.y);
            let y2 = -yb / ya - common.y;
            Ok((
                Point {
                    x: -(b * y2 + c) / a,
                    y: y2,
                },
                common,
            ))
        } else {
            let xa = b * b;
            let xb = -2.0 * b * b * O.x;
            let x2 = -xb / xa - common.x;
            Ok((Point { x: x2, y: -c / b }, common))
        }
    }
}

impl Intersect<Line> for Circle {
    type InterResult = (Point, Point);
    #[inline]
    fn inter(self, obj: Line) -> Result<Self::InterResult> {
        obj.inter(self)
    }
    #[inline]
    fn inter_common(self, obj: Line, common: Point) -> Result<Self::InterResult> {
        obj.inter_common(self, common)
    }
}

/// The radical axis of two Circles.
pub fn radical_axis(c: Circle, d: Circle) -> Line {
    let O = c.O;
    let P = d.O;
    let d1 = -2.0 * O.x;
    let e1 = -2.0 * O.y;
    let f1 = O.x * O.x + O.y * O.y - c.r * c.r;
    let d2 = -2.0 * P.x;
    let e2 = -2.0 * P.y;
    let f2 = P.x * P.x + P.y * P.y - d.r * d.r;
    Line {
        a: d1 - d2,
        b: e1 - e2,
        c: f1 - f2,
    }
}

impl Intersect<Circle> for Circle {
    type InterResult = (Point, Point);
    #[inline]
    fn inter(self, obj: Circle) -> Result<Self::InterResult> {
        radical_axis(self, obj).inter(obj)
    }
    #[inline]
    fn inter_common(self, obj: Circle, common: Point) -> Result<Self::InterResult> {
        radical_axis(self, obj).inter_common(obj, common)
    }
}

/// A trait for testing whether an object passes through an instance of `T`.
/// Provides the `is_through` method.
pub trait TestThrough<T> {
    fn is_through(self, P: T) -> bool;
}

impl TestThrough<Point> for Line {
    /// Test if the Line is through a Point.
    #[inline]
    fn is_through(self, P: Point) -> bool {
        aprx_eq(self.a * P.x + self.b * P.y + self.c, 0.0)
    }
}

impl TestThrough<Point> for Circle {
    /// Test if the Circle is through a Point.
    #[inline]
    fn is_through(self, p: Point) -> bool {
        aprx_eq(self.r * self.r, self.O.distance_sq(p))
    }
}
