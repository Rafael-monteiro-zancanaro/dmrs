use std::fmt::Display;

/// `CongruenceRelation`: Representation of a congruence.
///
/// Represents the following relation:
///
/// x ≡ y (mod n)
pub struct CongruenceRelation {
    /// `x`: Represents the dividend of the congruence.
    pub x: i32,
    /// `y`: Represents the remainder (or the equivalence class) of the congruence.
    pub y: i32,
    /// `modn`: Represents the divisor of the congruence.
    pub modn: i32,
}

impl CongruenceRelation {
    /// `CongruenceRelation::from`: Constructs the Congruence Representation by its parts.
    ///
    /// Given the `x`, `y` and `modn`, creates the representation of the following relation:
    ///
    /// x ≡ y (mod n)
    ///
    /// Example:
    /// ```rust
    /// use dmrs::number_theory::types::congruence_relation::CongruenceRelation;
    /// let congruence = CongruenceRelation::from(53, 23, 10);
    /// /*
    ///     'congruence' has the type:
    ///     
    ///     CongruenceRelation {
    ///         x: 53,
    ///         y: 23,
    ///         modn: 10,
    ///     };
    ///
    ///     In other words:
    ///     53 ≡ 23 (mod 10)
    /// */
    /// ```
    pub fn from(x: i32, y: i32, modn: i32) -> Self {
        return Self { x, y, modn };
    }

    /// `CongruenceRelation::is_valid`: Checks if a congruence is valid or not.
    ///
    /// Returns `true` if the congruence representation is valid, `false` otherwise.
    ///
    /// Examples:
    ///
    /// ```rust
    /// use dmrs::number_theory::types::congruence_relation::CongruenceRelation;
    /// let congruence = CongruenceRelation::from(152, 17, 10);
    /// let other_congruence = CongruenceRelation::from(152, 36, 5);
    /// println!("is valid? {}", congruence.is_valid()); // true, because 152 ≡ 17 (mod 10)
    /// println!("is valid? {}", other_congruence.is_valid()); // false, because 152 ≢ 36 (mod 5)
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        let is_multiple = ((self.x - self.y) % self.modn) == 0;
        return is_multiple;
    }
}

impl Display for CongruenceRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = if self.is_valid() {
            String::from("≡")
        } else {
            String::from("≢")
        };
        return write!(
            f,
            "{} {} {} (mod {})",
            self.x.to_string(),
            symbol,
            self.y.to_string(),
            self.modn.to_string()
        );
    }
}
