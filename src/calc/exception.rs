/// All kinds of exceptions that can occur in calculation.
#[derive(Debug, PartialEq, Eq)]
pub enum CalcException {
    /// Two points overlap when they shouldn't
    OverlappingPoint,
    /// Defining a circle with nonpositive radius
    NonpositiveRadius,
    /// Three (or more) points are collinear when they shouldn't
    CollinearPoints,
    /// There are no intersection
    NoIntersection,
    /// Defining a line with the coefficient of `x` and `y` being both zero
    ZeroCoefficient,
}

pub type Result<T, E = CalcException> = std::result::Result<T, E>;
