use std::fmt::Display;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Suit {
    pub fn from_str(s: &str) -> Suit {
        match s {
            "S" => Suit::Spade,
            "H" => Suit::Heart,
            "D" => Suit::Diamond,
            "C" => Suit::Club,
            _ => panic!("Invalid suit {}", s),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).chars().next().unwrap())
    }
}
