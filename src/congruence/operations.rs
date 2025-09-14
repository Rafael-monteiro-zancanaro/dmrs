use crate::congruence::types::euclidian_form::EuclideanForm;

pub fn discrete_div(dividend: i32, divider: i32) -> EuclideanForm {
    let remainder = dividend.rem_euclid(divider);
    let quocient = dividend.div_euclid(divider);
    return EuclideanForm::from(dividend, divider, quocient, remainder as u32);
}

pub fn modulo(dividend: i32, divider: i32) -> i32 {
    return dividend % divider;
}
