use crate::rank::Rank;
use crate::suit::Suit;
use std::fmt::Display;

#[derive(Eq, Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn from_str(s: &str) -> Card {
        let mut chars = s.chars();
        let rank = Rank::from_str(&chars.next().unwrap().to_string());
        let suit = Suit::from_str(&chars.next().unwrap().to_string());
        Card { rank, suit }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_cards() {
        assert!(Card::from_str("2S") < Card::from_str("3S"));
        assert_eq!(Card::from_str("2S"), Card::from_str("2D"));
    }
}
