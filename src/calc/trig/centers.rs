#![allow(non_snake_case)]

use crate::{
    calc::{
        basic::{Distance, Intersect},
        construct::{angle_bisect_3p, midpoint, perp, perp_bisect},
        exception::{CalcException, Result},
        transform::Reflect,
    },
    objects::{Line, Point},
};

use super::Triangle;

/// The isogonal conjugate of a point.
#[inline]
pub fn isogonal_conjugate((A, B, C): Triangle, P: Point) -> Result<Point> {
    let l1 = angle_bisect_3p(A, B, C)?.0;
    let l2 = angle_bisect_3p(A, C, B)?.0;
    let P1 = P.reflect_in(l1);
    let P2 = P.reflect_in(l2);
    Line::from_2p(B, P1)?.inter(Line::from_2p(C, P2)?)
}

/// Returns a point from its barycentric coordinates.
#[inline]
pub fn from_barycentric((A, B, C): Triangle, (x, y, z): (f64, f64, f64)) -> Result<Point> {
    let s = x + y + z;
    if s == 0.0 {
        Err(CalcException::ZeroCoefficient)
    } else {
        Ok((A * x + B * y + C * z) / s)
    }
}

#[inline]
pub fn circum((A, B, C): Triangle) -> Result<Point> {
    perp_bisect(A, B)?.inter(perp_bisect(A, C)?)
}

#[inline]
pub fn incenter((A, B, C): Triangle) -> Result<Point> {
    angle_bisect_3p(A, C, B)?
        .0
        .inter(angle_bisect_3p(A, B, C)?.0)
}

/// Returns the excenter **contained in the angle `BAC`**.
#[inline]
pub fn excenter((A, B, C): Triangle) -> Result<Point> {
    angle_bisect_3p(B, A, C)?
        .0
        .inter(angle_bisect_3p(A, B, C)?.1)
}

#[inline]
pub fn ortho((A, B, C): Triangle) -> Result<Point> {
    perp(A, Line::from_2p(B, C)?).inter(perp(B, Line::from_2p(A, C)?))
}

#[inline]
pub fn centroid((A, B, C): Triangle) -> Point {
    (A + B + C) / 3.0
}

#[inline]
pub fn nine_point((A, B, C): Triangle) -> Result<Point> {
    circum((midpoint(A, B), midpoint(C, B), midpoint(A, C)))
}

#[inline]
pub fn symmedian((A, B, C): Triangle) -> Result<Point> {
    let a2 = C.distance_sq(B);
    let b2 = B.distance_sq(A);
    let c2 = A.distance_sq(C);
    from_barycentric((A, B, C), (a2, b2, c2))
}

#[inline]
pub fn gergonne((A, B, C): Triangle) -> Result<Point> {
    let a = C.distance(B);
    let b = B.distance(A);
    let c = A.distance(C);
    let p = (a + b + c) / 2.0;
    from_barycentric((A, B, C), (p - a, p - b, p - c))
}
