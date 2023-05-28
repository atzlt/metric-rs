#![allow(non_snake_case)]

use metric::{
    objects::{Point, Line, Circle},
    calc::{basic::{Intersect, is_parallel, TestThrough}, exception::CalcException},
};

#[test]
fn objects_def() {
    let A = Point::new(0.0, 0.0);
    let B = Point::new(0.0, 1.0);
    let C = Point::new(1.0, 0.0);
    let D = Point::new(1.0, 1.0);
    let m = Line::from_2p(B, C).unwrap();
    let n = Line::from_2p(A, D).unwrap();
    let k = Line::from_slope_and_point(2.0, -1.0, D);
    assert_eq!(k.b, -1.0);
    let O = m.inter(n).unwrap();
    assert!(O == Point::new(0.5, 0.5));
    let c = Circle::from_3p(A, B, D).unwrap();
    let c0 = Circle::from_center_point(O, A).unwrap();
    assert!(c == c0);
    assert_eq!(c.r, (0.5f64).sqrt());
    let E = Point::new(0.0, 1.0);
    assert_eq!(Line::from_2p(B, E).unwrap_err(), CalcException::OverlappingPoint);
    assert_eq!(Circle::from_3p(A, B, E).unwrap_err(), CalcException::OverlappingPoint);
    assert_eq!(Circle::from_3p(A, O, D).unwrap_err(), CalcException::NoIntersection);
    assert_eq!(Circle::from_center_radius(O, 0.0).unwrap_err(), CalcException::NonpositiveRadius);
}

#[test]
fn predicates() {
    let A = Point::new(0.0, 0.0);
    let C = Point::new(10.0, 24.0);
    let D = Point::new(100.0, 240.000001);
    let E = Point::new(10.0, 24.000001);
    let l = Line::from_2p(A, C).unwrap();
    let l0 = Line::from_2p(A, D).unwrap();
    let k = Line::from_2p(E, D).unwrap();
    let c = Circle::from_center_radius(A, 26.0).unwrap();
    assert!(is_parallel(l, k));
    assert!(c.is_through(C));
    assert!(!l0.is_through(C));
}
