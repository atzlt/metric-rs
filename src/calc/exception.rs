use std::fmt::Display;

/// All kinds of exceptions that can occur in calculation.
/// Most of them are about preventing division by zero, or other
/// cases that are not well-defined, like "the" line through two
/// overlapping points.
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
    /// Defining something with coefficient all zero, where this is not allowed
    ZeroCoefficient,
    /// Some calculation would directly cause the result to be the point of
    /// infinity / the line of infinity, like the homothetic center of two
    /// equal circles.
    Infinity,
}

pub type Result<T, E = CalcException> = std::result::Result<T, E>;

impl Display for CalcException {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcException::CollinearPoints => write!(
                f,
                "Three (or more) points are collinear when they shouldn't"
            ),
            CalcException::NoIntersection => write!(f, "There are no intersection"),
            CalcException::NonpositiveRadius => {
                write!(f, "Defining a circle with nonpositive radius")
            }
            CalcException::OverlappingPoint => write!(f, "Two points overlap when they shouldn't"),
            CalcException::ZeroCoefficient => write!(
                f,
                "Defining something with coefficient all zero, where this is not allowed"
            ),
            CalcException::Infinity => write!(
                f,
                "This calculation would directly cause the result to be the point of infinity / the line of infinity"
            ),
        }
    }
}
