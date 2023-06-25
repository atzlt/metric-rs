#![allow(non_snake_case)]

use crate::objects::{Circle, Line, Point};

use super::{
    basic::{Intersect, TestThrough},
    exception::{CalcException, Result},
};

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

/// Construct the polar line of a point w.r.t. a circle.
#[inline]
pub fn polar_line(A: Point, c: Circle) -> Result<Line> {
    let Point { x: x0, y: y0 } = A;
    let Circle { O, r } = c;
    let Point { x: a, y: b } = O;
    Line::from_coeff(x0 - a, y0 - b, a * (a - x0) + b * (b - y0) - r * r)
}

/// Construct the tangent through a point.
#[inline]
pub fn tangent(A: Point, c: Circle) -> Result<(Line, Line)> {
    if c.is_through(A) {
        let l = perp(A, Line::from_2p(A, c.O)?);
        Ok((l, l))
    } else {
        let (P, Q) = c.inter(polar_line(A, c)?)?;
        Ok((Line::from_2p(A, P)?, Line::from_2p(A, Q)?))
    }
}

/// Construct the homothetic center of two circles. The first being
/// the outer one, the last the inner.
#[inline]
pub fn homothety_center(c: Circle, d: Circle) -> Result<(Point, Point)> {
    if c.r == d.r {
        Err(CalcException::Infinity)
    } else {
        let r1 = c.r;
        let r2 = d.r;
        Ok((
            (c.O * r2 + d.O * r1) / (r1 + r2),
            (d.O * r1 - c.O * r2) / (r1 - r2),
        ))
    }
}

/// Construct the two outer common tangents of two circles.
#[inline]
pub fn outer_common_tangent(c: Circle, d: Circle) -> Result<(Line, Line)> {
    let (O, _) = homothety_center(c, d)?;
    tangent(O, c)
}


/// Construct the two inner common tangents of two circles.
#[inline]
pub fn inner_common_tangent(c: Circle, d: Circle) -> Result<(Line, Line)> {
    let (_, O) = homothety_center(c, d)?;
    tangent(O, c)
}
