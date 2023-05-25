use crate::objects::{Circle, Line, Point};

use super::constructs::perp_bisect;

const EPSILON: f64 = 1e-10;
pub const DEG: f64 = std::f64::consts::PI / 180.0;
pub const ROUND: f64 = 2.0 * std::f64::consts::PI;

#[inline]
fn aprx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[inline]
pub fn is_parallel(l: Line, k: Line) -> bool {
    aprx_eq(l.a * k.b, l.b * k.a)
}

pub trait DistanceTo<T>
where
    Self: Sized,
{
    /// The square of the distance.
    fn distance_sq(self, obj: T) -> f64;
    #[inline]
    fn distance(self, obj: T) -> f64 {
        self.distance_sq(obj).sqrt()
    }
}

impl DistanceTo<Point> for Point {
    fn distance_sq(self, obj: Point) -> f64 {
        let dx = self.x - obj.x;
        let dy = self.y - obj.y;
        dx * dx + dy * dy
    }
}

impl DistanceTo<Line> for Point {
    fn distance_sq(self, obj: Line) -> f64 {
        let z = self.x * obj.a + self.y * obj.b + obj.c;
        z * z / (obj.a * obj.a + obj.b * obj.b)
    }
}

impl DistanceTo<Line> for Line {
    fn distance_sq(self, obj: Line) -> f64 {
        if !is_parallel(self, obj) {
            0.0
        } else {
            let z = self.c - obj.c;
            z * z / (self.a * self.a + self.b * self.b)
        }
    }
}

/// The angle between two lines, the one in `[0, pi / 2]`.
pub fn angle(l: Line, k: Line) -> f64 {
    let (a, b) = (l.a, l.b);
    let (c, d) = (k.a, k.b);
    let a0 = a * a + b * b;
    let b0 = c * c + d * d;
    let p = (a * c + b * d) / (a0 * b0).sqrt();
    p.abs().acos()
}

impl std::cmp::PartialEq for Point {
    /// If two Points are _approximately_ equal.
    /// Here _approximately_ roughly means `1e-10` error.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        aprx_eq(self.x, other.x) && aprx_eq(self.y, other.y)
    }
}

impl std::cmp::PartialEq for Line {
    /// If two Lines _approximately_ overlaps.
    /// Here _approximately_ roughly means `1e-10` error.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        is_parallel(*self, *other) && aprx_eq(self.c, other.c)
    }
}

impl std::cmp::PartialEq for Circle {
    /// If two Circles _approximately_ overlaps.
    /// Here _approximately_ roughly means `1e-10` error.
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.o == other.o && aprx_eq(self.r, other.r)
    }
}

/// A trait for constructing intersections.
pub trait Intersect<T> {
    /// The result of intersection.
    /// This is generally `Option<Point>`, or a tuple of `Option<Point>`.
    /// Use `Option` because there might be no intersections.
    type InterResult;
    /// Intersection.
    fn inter(self, obj: T) -> Self::InterResult;
    /// Intersection _with a common point given_.
    /// This can simplify calculation (using Vieta's theorem).
    /// **Do notice that** this common point will _directly_ affect the result.
    /// If a wrong common point is given the result will be _totally wrong_.
    /// The given common point is always the last element of the tuple.
    fn inter_common(self, _: T, common: Point) -> Self::InterResult;
}

impl Intersect<Line> for Line {
    type InterResult = Option<Point>;
    fn inter(self, obj: Line) -> Self::InterResult {
        if is_parallel(self, obj) {
            None
        } else {
            let a = self.b * obj.c - obj.b * self.c;
            let b = self.c * obj.a - obj.c * self.a;
            let d = self.a * obj.b - obj.a * self.b;
            Some(Point { x: a / d, y: b / d })
        }
    }
    #[inline]
    fn inter_common(self, _: Line, common: Point) -> Self::InterResult {
        Some(common)
    }
}

impl Intersect<Circle> for Line {
    type InterResult = (Option<Point>, Option<Point>);
    fn inter(self, obj: Circle) -> Self::InterResult {
        let Circle { o, r } = obj;
        let Line { a, b, c } = self;
        if a != 0.0 {
            let ya = a * a + b * b;
            let yb = 2.0 * ((a * o.x + c) * b - a * a * o.y);
            let yc = a * a * (o.x * o.x - r * r) + (a * o.x + c) * (a * o.x + c);
            let disc = yb * yb - 4.0 * ya * yc;
            if disc < 0.0 {
                return (None, None);
            }
            let y1 = (-yb + disc) / ya / 2.0;
            let y2 = (-yb - disc) / ya / 2.0;
            (
                Some(Point {
                    x: -(b * y1 + c) / a,
                    y: y1,
                }),
                Some(Point {
                    x: -(b * y2 + c) / a,
                    y: y2,
                }),
            )
        } else {
            let xa = b * b;
            let xb = -2.0 * b * b * o.x;
            let xc = b * b * (o.x * o.x - r * r) + (b * o.x + c) * (b * o.x + c);
            let disc = xb * xb - 4.0 * xa * xc;
            if disc < 0.0 {
                return (None, None);
            }
            let x1 = (-xb + disc) / xa / 2.0;
            let x2 = (-xb - disc) / xa / 2.0;
            (
                Some(Point { x: x1, y: -c / b }),
                Some(Point { x: x2, y: -c / b }),
            )
        }
    }
    fn inter_common(self, obj: Circle, common: Point) -> Self::InterResult {
        let o = obj.o;
        let Line { a, b, c } = self;
        if a != 0.0 {
            let ya = a * a + b * b;
            let yb = 2.0 * ((a * o.x + c) * b - a * a * o.y);
            let y2 = -yb / ya - common.y;
            (
                Some(Point {
                    x: -(b * y2 + c) / a,
                    y: y2,
                }),
                Some(common),
            )
        } else {
            let xa = b * b;
            let xb = -2.0 * b * b * o.x;
            let x2 = -xb / xa - common.x;
            (Some(Point { x: x2, y: -c / b }), Some(common))
        }
    }
}

impl Intersect<Line> for Circle {
    type InterResult = (Option<Point>, Option<Point>);
    #[inline]
    fn inter(self, obj: Line) -> Self::InterResult {
        obj.inter(self)
    }
    #[inline]
    fn inter_common(self, obj: Line, common: Point) -> Self::InterResult {
        obj.inter_common(self, common)
    }
}

/// The radical axis of two Circles.
pub fn radical_axis(c: Circle, d: Circle) -> Line {
    let o = c.o;
    let p = d.o;
    let d1 = -2.0 * o.x;
    let e1 = -2.0 * o.y;
    let f1 = o.x * o.x + o.y * o.y - c.r * c.r;
    let d2 = -2.0 * p.x;
    let e2 = -2.0 * p.y;
    let f2 = p.x * p.x + p.y * p.y - d.r * d.r;
    Line {
        a: d1 - d2,
        b: e1 - e2,
        c: f1 - f2,
    }
}

impl Intersect<Circle> for Circle {
    type InterResult = (Option<Point>, Option<Point>);
    #[inline]
    fn inter(self, obj: Circle) -> Self::InterResult {
        radical_axis(self, obj).inter(obj)
    }
    #[inline]
    fn inter_common(self, obj: Circle, common: Point) -> Self::InterResult {
        radical_axis(self, obj).inter_common(obj, common)
    }
}

impl Point {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

impl Line {
    /// Construct new Line from coefficients: `ax + by + c = 0`.
    #[inline]
    pub fn from_coeff(a: f64, b: f64, c: f64) -> Line {
        Line { a, b, c }
    }
    /// Construct new Line from two coefficients `ax + by + ?? = 0` and
    /// a given point that the line passes.
    #[inline]
    pub fn from_slope_and_point(a: f64, b: f64, p: Point) -> Self {
        Line {
            a,
            b,
            c: -a * p.x - b * p.y,
        }
    }
    /// Construct new Line passing through two Points.
    /// If the two Points overlap return `OverlappingPoint` error.
    #[inline]
    pub fn from_2p(a: Point, b: Point) -> Result<Self, GObjectConstructionErr> {
        if a == b {
            Err(GObjectConstructionErr::OverlappingPoint)
        } else {
            Ok(Line {
                a: a.y - b.y,
                b: b.x - a.x,
                c: a.y * (a.x - b.x) - a.x * (a.y * b.y),
            })
        }
    }

    /// Test if the Line is through a Point.
    #[inline]
    pub fn is_through(self, p: Point) -> bool {
        aprx_eq(self.a * p.x + self.b * p.y + self.c, 0.0)
    }
}

impl Circle {
    /// Construct a Circle with center `o` and radius `r`.
    /// If the radius given is nonpositive return `NonpositiveRadius` error.
    #[inline]
    pub fn from_center_radius(o: Point, r: f64) -> Result<Self, GObjectConstructionErr> {
        if r <= 0.0 {
            Err(GObjectConstructionErr::NonpositiveRadius)
        } else {
            Ok(Circle { o, r })
        }
    }
    /// Construct a Circle with center `o` and a point it passes through `a`.
    #[inline]
    pub fn from_center_point(o: Point, a: Point) -> Result<Self, GObjectConstructionErr> {
        if o == a {
            Err(GObjectConstructionErr::NonpositiveRadius)
        } else {
            Ok(Circle {
                o,
                r: o.distance(a),
            })
        }
    }
    /// Construct a Circle passing through three Points.
    /// If any two of them overlap return `OverlappingPoint` error.
    pub fn from_3p(a: Point, b: Point, c: Point) -> Result<Self, GObjectConstructionErr> {
        if a == b || b == c || c == a {
            Err(GObjectConstructionErr::OverlappingPoint)
        } else {
            let o = perp_bisect(a, b).inter(perp_bisect(b, c));
            if o == None {
                Err(GObjectConstructionErr::CollinearPoints)
            } else {
                let o = o.unwrap();
                let r = o.distance(a);
                Ok(Circle { o, r })
            }
        }
    }

    /// Test if the Line is through a Point.
    #[inline]
    pub fn is_through(self, p: Point) -> bool {
        aprx_eq(self.r * self.r, self.o.distance_sq(p))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GObjectConstructionErr {
    OverlappingPoint,
    NonpositiveRadius,
    CollinearPoints,
}
