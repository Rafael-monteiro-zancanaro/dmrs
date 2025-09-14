use std::fmt::Display;

pub struct CongruenceRelation {
    pub x: i32,
    pub y: i32,
    pub modn: i32,
}

impl CongruenceRelation {
    pub fn from(x: i32, y: i32, modn: i32) -> Self {
        return Self { x, y, modn };
    }

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
