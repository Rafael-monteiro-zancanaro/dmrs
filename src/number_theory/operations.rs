use crate::number_theory::types::euclidian_form::EuclideanForm;

/// `euclidean_representation_of`: Retrieve the Euclidian Representation of a division in ℤ.
///
/// This method returns the **Euclidean representation** of a division given the dividend and divisor.
///
/// The **Euclidean Representation** of a division (a/b) is:
///
/// `a = b * q + r`
///
/// Where:
/// - `a`: the dividend (number to be divided)
/// - `b`: the divisor (number that divides)
/// - `q`: the quotient (largest integer such that `b * q <= a`)
/// - `r`: the remainder (`a - (b * q)`), always non-negative with `0 <= r < |b|`
///
/// Example:
/// ```rust
/// use dmrs::number_theory::operations::euclidean_representation_of;
///
/// let euclidean_result = euclidean_representation_of(11_i32, 3_i32);
/// /*
///  'euclidean_result' has the type:
///     EuclideanForm {
///         dividend: 11,
///         divisor: 3,
///         quotient: 3,
///         remainder: 2
///     };
/// */
///
/// println!("{}", euclidean_result); // Will print "11 = 3 ⋅ 3 + 2"
/// ```
pub fn euclidean_representation_of(dividend: i32, divider: i32) -> EuclideanForm {
    let remainder = dividend.rem_euclid(divider);
    let quotient = dividend.div_euclid(divider);
    return EuclideanForm::from(dividend, divider, quotient, remainder as u32);
}

/// `modulo`: Retrieve the remainder of a division in ℤ.
///
/// This method returns the **Remainder** of a division directly.
///
/// Where:
/// - `dividend`: Number to be divided;
/// - `divisor`: Number that divides.
///
/// Example:
/// ```rust
///
/// use dmrs::number_theory::operations::modulo;
/// let remainder = modulo(152, 5);
/// assert_eq!(remainder, 2_i32); // true, because 152 mod 5 == 2
/// ```
pub fn modulo(dividend: i32, divider: i32) -> i32 {
    return dividend.rem_euclid(divider);
}

/// `div`: Retrieve the remainder of a division in ℤ.
///
/// This method returns the **Biggest Quotient** of a division directly.
///
/// Where:
/// - `dividend`: Number to be divided;
/// - `divisor`: Number that divides.
///
/// Example:
/// ```rust
/// use dmrs::number_theory::operations::div;
/// let quotient = div(152, 5);
/// assert_eq!(quotient, 30_i32); // true, because 152 div 5 == 30
/// ```
pub fn div(dividend: i32, divider: i32) -> i32 {
    return dividend.div_euclid(divider);
}
