#![allow(non_snake_case)]

/// A struct representing a Point.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} , {} )", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    /// Perform vector addition.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    /// Perform vector subtraction.
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    /// Perform scalar multiplication.
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    /// Perform scalar division.
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// A struct representing a Line, by its standard form `Ax + By + C = 0`.
#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x + {}y + {}", self.a, self.b, self.c)
    }
}

/// A struct representing a Circle, by its center and radius.
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub O: Point,
    pub r: f64,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "circ({}, {})", self.O, self.r)
    }
}
