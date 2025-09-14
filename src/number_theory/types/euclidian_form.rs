use std::fmt::Display;

/// `EuclideanForm`: Euclidean Representation of a division.
///
/// Represents the following relation:
///
/// a = b ⋅ q + r
///
/// Where:
/// - `a`: the dividend (number to be divided)
/// - `b`: the divisor (number that divides)
/// - `q`: the quotient (largest integer such that `b * q <= a`)
/// - `r`: the remainder (`a - (b * q)`), always non-negative with `0 <= r < |b|`
pub struct EuclideanForm {
    /// `dividend`: Number to be divided
    pub dividend: i32,
    /// `divisor`: Number that divides
    pub divisor: i32,
    /// `quotient`: Largest integer such that `b * q <= a`
    pub quotient: i32,
    /// `remainder`: (`a - (b * q)`), always non-negative with `0 <= r < |b|`
    pub remainder: u32,
}

impl EuclideanForm {
    /// `EuclideanForm::from`: Constructs the Euclidean representation of a division by its parts.
    ///
    /// Example:
    /// ```rust
    /// let euclidean_division = EuclideanForm::from(-37, 5, -8, 3)
    /// /*
    ///     'euclidean_division' has the type:
    ///     EuclideanForm {
    ///         dividend: -37,
    ///         divider: 5,
    ///         quotient: -8,
    ///         remainder: 3,
    ///     }
    ///     
    ///     In other words:
    ///     -37 = 5 ⋅ (-8) + 3
    /// */
    ///
    pub fn from(dividend: i32, divider: i32, quotient: i32, remainder: u32) -> Self {
        return Self {
            dividend,
            divisor: divider,
            quotient,
            remainder,
        };
    }
}

impl Display for EuclideanForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = ({} ⋅ {}) + {}",
            self.dividend.to_string(),
            self.divisor.to_string(),
            self.quotient.to_string(),
            self.remainder.to_string()
        )
    }
}
