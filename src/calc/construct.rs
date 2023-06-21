#![allow(non_snake_case)]

use crate::objects::{Line, Point};

use super::exception::Result;

/// Construct midpoint.
#[inline]
pub fn midpoint(A: Point, B: Point) -> Point {
    (A + B) / 2.0
}

/// Construct center of polygon.
pub fn center(poly: &Vec<Point>) -> Point {
    let mut s = Point { x: 0.0, y: 0.0 };
    for p in poly {
        s = s + *p;
    }
    s / poly.len() as f64
}

/// Construct parallel line through a Point.
#[inline]
pub fn parallel(A: Point, l: Line) -> Line {
    Line {
        a: l.a,
        b: l.b,
        c: -(l.a * A.x + l.b * A.y),
    }
}

/// Construct perpendicular line through a Point.
#[inline]
pub fn perp(A: Point, l: Line) -> Line {
    Line {
        a: -l.b,
        b: l.a,
        c: l.b * A.x - l.a * A.y,
    }
}

/// Construct the projection of a Point on a Line.
pub fn projection(A: Point, l: Line) -> Point {
    let Line { a, b, c } = l;
    let n = a * a + b * b;
    Point {
        x: (b * b * A.x - a * c - a * b * A.y) / n,
        y: (a * a * A.y - b * c - a * b * A.x) / n,
    }
}

/// Construct the perpendicular bisector of two points.
/// **Panicks when two points are the same.**
#[inline]
pub fn perp_bisect(A: Point, B: Point) -> Result<Line> {
    Ok(perp(midpoint(A, B), Line::from_2p(A, B)?))
}

/// Constructs the two angle bisectors of two lines.
pub fn angle_bisect(l: Line, k: Line) -> (Line, Line) {
    let Line { a, b, c } = l;
    let Line { a: e, b: f, c: g } = k;
    let m = (a * a + b * b).sqrt();
    let n = (e * e + f * f).sqrt();
    let (a0, b0, c0) = (a / m, b / m, c / m);
    let (a1, b1, c1) = (e / n, f / n, g / n);
    (
        Line {
            a: a0 + a1,
            b: b0 + b1,
            c: c0 + c1,
        },
        Line {
            a: a0 - a1,
            b: b0 - b1,
            c: c0 - c1,
        },
    )
}

/// Constructs the two angle bisectors of an angle, interior first, exterior second.
#[inline]
pub fn angle_bisect_3p(A: Point, O: Point, B: Point) -> Result<(Line, Line)> {
    Ok(angle_bisect(Line::from_2p(O, A)?, Line::from_2p(O, B)?))
}
