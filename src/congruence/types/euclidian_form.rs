//TODO: Maybe create an EuclidianFormBuilder, to create a Euclidian form by its fields

use std::fmt::Display;

pub struct EuclideanForm {
    pub dividend: i32,
    pub divider: i32,
    pub quocient: i32,
    pub remainder: u32,
}

impl EuclideanForm {
    pub fn from(dividend: i32, divider: i32, quocient: i32, remainder: u32) -> Self {
        return Self {
            dividend,
            divider,
            quocient,
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
            self.divider.to_string(),
            self.quocient.to_string(),
            self.remainder.to_string()
        )
    }
}
