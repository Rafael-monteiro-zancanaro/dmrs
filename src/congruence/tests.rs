#[cfg(test)]
mod discrete_div_tests {
    use crate::congruence::operations::discrete_div;

    #[test]
    pub fn should_divide_discretly() {
        let division = discrete_div(11_i32, 3_i32);
        assert_eq!(division.quocient, 3_i32);
        assert_eq!(division.remainder, 2_u32);
    }

    #[test]
    pub fn remainder_should_be_always_positive() -> () {
        let division = discrete_div(-37_i32, 5_i32);
        assert_eq!(division.quocient, -8_i32);
        assert_eq!(division.remainder, 3_u32);
    }

    #[test]
    pub fn should_display_correctly() -> () {
        let division = discrete_div(-37_i32, 5_i32);
        let definition = format!("{}", division);
        assert_eq!("-37 = (5 ⋅ -8) + 3", &definition);
    }
}

#[cfg(test)]
mod congruence_relation_tests {
    use crate::congruence::types::congruence_relation::CongruenceRelation;

    #[test]
    pub fn should_not_be_a_valid_congruence() -> () {
        let congruence_or_not_so = CongruenceRelation::from(3, 12, 2);
        assert_eq!(congruence_or_not_so.is_valid(), false);
    }

    #[test]
    pub fn congruence_should_be_valid() -> () {
        let congruence = CongruenceRelation::from(0, -14, 2);
        assert_eq!(congruence.is_valid(), true);
    }

    #[test]
    pub fn should_display_correctly() -> () {
        let congruence = CongruenceRelation::from(3, 15, 2);
        let definition = format!("{}", congruence);
        assert_eq!("3 ≡ 15 (mod 2)", &definition)
    }

    #[test]
    pub fn should_display_incorrect_congruence_correctly() -> () {
        let congruence = CongruenceRelation::from(3, 12, 2);
        let definition = format!("{}", congruence);
        assert_eq!("3 ≢ 12 (mod 2)", &definition)
    }
}
