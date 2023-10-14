use crate::card::Card;
use crate::poker_hand::PokerHand;

#[derive(Debug)]
pub struct Hand {
    pub card1: Card,
    pub card2: Card,
    pub card3: Card,
    pub card4: Card,
    pub card5: Card,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        PokerHand::get_poker_hand(self) == PokerHand::get_poker_hand(other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        let poker_hand_1 = PokerHand::get_poker_hand(self);
        let poker_hand_2 = PokerHand::get_poker_hand(other);
        poker_hand_1.partial_cmp(&poker_hand_2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let (h1, h2) = hands("5H 5C 6S 7S KD", "2C 3S 8S 8D TD");
        assert!(h1 < h2);

        let (h1, h2) = hands("5D 8C 9S JS AC", "2C 5C 7D 8S QH");
        assert!(h1 > h2);

        let (h1, h2) = hands("2D 9C AS AH AC", "3D 6D 7D TD QD");
        assert!(h1 < h2);

        let (h1, h2) = hands("4D 6S 9H QH QC", "3D 6D 7H QD QS");
        assert!(h1 > h2);

        let (h1, h2) = hands("2H 2D 4C 4D 4S", "3C 3D 3S 9S 9D");
        assert!(h1 > h2);

        let (h1, h2) = hands("6D 7C 5D 5H 3S", "5C JC 2H 5S 3D");
        assert!(h1 < h2);
    }

    fn hands(h1: &str, h2: &str) -> (Hand, Hand) {
        let h1 = h1
            .split_whitespace()
            .map(Card::from_str)
            .collect::<Vec<Card>>();
        let h1 = Hand {
            card1: h1[0],
            card2: h1[1],
            card3: h1[2],
            card4: h1[3],
            card5: h1[4],
        };
        let h2 = h2
            .split_whitespace()
            .map(Card::from_str)
            .collect::<Vec<Card>>();
        let h2 = Hand {
            card1: h2[0],
            card2: h2[1],
            card3: h2[2],
            card4: h2[3],
            card5: h2[4],
        };
        (h1, h2)
    }
}
