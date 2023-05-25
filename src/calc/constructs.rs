use crate::objects::{Line, Point};

/// Construct midpoint.
#[inline]
pub fn midpoint(a: Point, b: Point) -> Point {
    (a + b) / 2.0
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
pub fn parallel(a: Point, l: Line) -> Line {
    Line {
        a: l.a,
        b: l.b,
        c: -(l.a * a.x + l.b * a.y),
    }
}

/// Construct perpendicular line through a Point.
#[inline]
pub fn perp(a: Point, l: Line) -> Line {
    Line {
        a: -l.b,
        b: l.a,
        c: l.b * a.x - l.a * a.y,
    }
}

/// Construct the projection of a Point on a Line.
pub fn projection(p: Point, l: Line) -> Point {
    let Line { a, b, c } = l;
    let n = a * a + b * b;
    Point {
        x: (b * b * p.x - a * c - a * b * p.y) / n,
        y: (a * a * p.y - b * c - a * b * p.x) / n,
    }
}

/// Construct the perpendicular bisector of two points.
/// **Panicks when two points are the same.**
#[inline]
pub fn perp_bisect(a: Point, b: Point) -> Line {
    perp(midpoint(a, b), Line::from_2p(a, b).unwrap())
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
/// **Panicks if any point is overlapping with `o`.**
#[inline]
pub fn angle_bisect_3p(a: Point, o: Point, b: Point) -> (Line, Line) {
    angle_bisect(Line::from_2p(o, a).unwrap(), Line::from_2p(o, b).unwrap())
}
