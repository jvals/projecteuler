use crate::card::Card;
use crate::hand::Hand;
use crate::rank::Rank;
use crate::rank::Rank::Ace;
use std::fmt::{Display, Formatter};

#[repr(u8)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum PokerHand {
    HighCard {
        card1: Card,
        card2: Card,
        card3: Card,
        card4: Card,
        card5: Card,
    },
    OnePair {
        pair: (Card, Card),
        card3: Card,
        card4: Card,
        card5: Card,
    },
    TwoPair {
        high_pair: (Card, Card),
        low_pair: (Card, Card),
        kicker: Card,
    },
    ThreeOfAKind {
        triple: (Card, Card, Card),
        card4: Card,
        card5: Card,
    },
    Straight {
        card1: Card,
        card2: Card,
        card3: Card,
        card4: Card,
        card5: Card,
    },
    Flush {
        card1: Card,
        card2: Card,
        card3: Card,
        card4: Card,
        card5: Card,
    },
    FullHouse {
        triple: (Card, Card, Card),
        pair: (Card, Card),
    },
    FourOfAKind {
        quad: (Card, Card, Card, Card),
        kicker: Card,
    },
    StraightFlush {
        card1: Card,
        card2: Card,
        card3: Card,
        card4: Card,
        card5: Card,
    },
    RoyalFlush,
}

impl Display for PokerHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PokerHand::HighCard {
                card1,
                card2,
                card3,
                card4,
                card5,
            } => {
                write!(
                    f,
                    "High card: {} {} {} {} {}",
                    card1, card2, card3, card4, card5
                )
            }
            PokerHand::OnePair {
                pair,
                card3,
                card4,
                card5,
            } => {
                write!(
                    f,
                    "One pair: {} {}, Kickers: {} {} {}",
                    pair.0, pair.1, card3, card4, card5
                )
            }
            PokerHand::TwoPair {
                high_pair,
                low_pair,
                kicker,
            } => {
                write!(
                    f,
                    "Two pair: {} {} {} {}, Kicker: {}",
                    high_pair.0, high_pair.1, low_pair.0, low_pair.1, kicker
                )
            }
            PokerHand::ThreeOfAKind {
                triple,
                card4,
                card5,
            } => {
                write!(
                    f,
                    "Three of a kind: {} {} {}, Kickers: {} {}",
                    triple.0, triple.1, triple.2, card4, card5
                )
            }
            PokerHand::Straight {
                card1,
                card2,
                card3,
                card4,
                card5,
            } => {
                write!(
                    f,
                    "Straight: {} {} {} {} {}",
                    card1, card2, card3, card4, card5
                )
            }
            PokerHand::Flush {
                card1,
                card2,
                card3,
                card4,
                card5,
            } => {
                write!(
                    f,
                    "Flush: {} {} {} {} {}",
                    card1, card2, card3, card4, card5
                )
            }
            PokerHand::FullHouse { triple, pair } => {
                write!(
                    f,
                    "Full house: {} {} {}, {} {}",
                    triple.0, triple.1, triple.2, pair.0, pair.1
                )
            }
            PokerHand::FourOfAKind { quad, kicker } => {
                write!(
                    f,
                    "Four of a kind: {} {} {} {}, Kicker: {}",
                    quad.0, quad.1, quad.2, quad.3, kicker
                )
            }
            PokerHand::StraightFlush {
                card1,
                card2,
                card3,
                card4,
                card5,
            } => {
                write!(
                    f,
                    "Straight flush: {} {} {} {} {}",
                    card1, card2, card3, card4, card5
                )
            }
            PokerHand::RoyalFlush => {
                write!(f, "Royal flush")
            }
        }
    }
}

impl PokerHand {
    pub fn get_poker_hand(hand: &Hand) -> PokerHand {
        if let Some(poker_hand) = PokerHand::royal_flush(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::straight_flush(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::four_of_a_kind(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::full_house(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::flush(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::straight(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::three_of_a_kind(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::two_pair(hand) {
            poker_hand
        } else if let Some(poker_hand) = PokerHand::one_pair(hand) {
            poker_hand
        } else {
            PokerHand::high_card(hand)
        }
    }

    fn high_card(hand: &Hand) -> PokerHand {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        cards.reverse();
        PokerHand::HighCard {
            card1: cards[0],
            card2: cards[1],
            card3: cards[2],
            card4: cards[3],
            card5: cards[4],
        }
    }

    fn one_pair(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        for i in 0..4 {
            if cards[i].rank == cards[i + 1].rank {
                let pair = (cards[i], cards[i + 1]);
                let kickers: Vec<Card> = vec![cards[0], cards[1], cards[2], cards[3], cards[4]]
                    .into_iter()
                    .filter(|card| *card != pair.0 && *card != pair.1)
                    .collect();
                return Some(PokerHand::OnePair {
                    pair,
                    card3: kickers[2],
                    card4: kickers[1],
                    card5: kickers[0],
                });
            }
        }
        None
    }

    fn two_pair(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        cards.reverse();
        for i in 0..4 {
            if cards[i].rank == cards[i + 1].rank {
                let high_pair = (cards[i], cards[i + 1]);
                for j in 0..4 {
                    if cards[j].rank == cards[j + 1].rank && cards[j].rank != high_pair.0.rank {
                        let low_pair = (cards[j], cards[j + 1]);
                        let kicker = vec![cards[0], cards[1], cards[2], cards[3], cards[4]]
                            .into_iter()
                            .find(|&card| {
                                card.rank != high_pair.0.rank && card.rank != low_pair.0.rank
                            })
                            .unwrap();
                        return Some(PokerHand::TwoPair {
                            high_pair,
                            low_pair,
                            kicker,
                        });
                    }
                }
            }
        }
        None
    }

    fn three_of_a_kind(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        cards.reverse();
        for i in 0..3 {
            if cards[i].rank == cards[i + 1].rank && cards[i].rank == cards[i + 2].rank {
                let triple = (cards[i], cards[i + 1], cards[i + 2]);
                let kickers: Vec<Card> = vec![cards[0], cards[1], cards[2], cards[3], cards[4]]
                    .into_iter()
                    .filter(|card| *card != triple.0 && *card != triple.1 && *card != triple.2)
                    .collect();
                return Some(PokerHand::ThreeOfAKind {
                    triple,
                    card4: kickers[0],
                    card5: kickers[1],
                });
            }
        }
        None
    }

    fn straight(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        if cards[0].rank.next() == Some(cards[1].rank)
            && cards[1].rank.next() == Some(cards[2].rank)
            && cards[2].rank.next() == Some(cards[3].rank)
            && cards[3].rank.next() == Some(cards[4].rank)
        {
            Some(PokerHand::Straight {
                card1: cards[4],
                card2: cards[3],
                card3: cards[2],
                card4: cards[1],
                card5: cards[0],
            })
        } else if cards[0].rank == Rank::Two
            && cards[1].rank == Rank::Three
            && cards[2].rank == Rank::Four
            && cards[3].rank == Rank::Five
            && cards[4].rank == Ace
        {
            Some(PokerHand::Straight {
                card1: cards[3],
                card2: cards[2],
                card3: cards[1],
                card4: cards[0],
                card5: cards[4],
            })
        } else {
            None
        }
    }

    fn flush(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        cards.reverse();
        if cards[0].suit == cards[1].suit
            && cards[1].suit == cards[2].suit
            && cards[2].suit == cards[3].suit
            && cards[3].suit == cards[4].suit
        {
            Some(PokerHand::Flush {
                card1: cards[0],
                card2: cards[1],
                card3: cards[2],
                card4: cards[3],
                card5: cards[4],
            })
        } else {
            None
        }
    }

    fn full_house(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        cards.reverse();
        if cards[0].rank == cards[1].rank
            && cards[1].rank == cards[2].rank
            && cards[3].rank == cards[4].rank
        {
            Some(PokerHand::FullHouse {
                triple: (cards[0], cards[1], cards[2]),
                pair: (cards[3], cards[4]),
            })
        } else if cards[0].rank == cards[1].rank
            && cards[2].rank == cards[3].rank
            && cards[3].rank == cards[4].rank
        {
            Some(PokerHand::FullHouse {
                triple: (cards[2], cards[3], cards[4]),
                pair: (cards[0], cards[1]),
            })
        } else {
            None
        }
    }

    fn four_of_a_kind(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        cards.reverse();
        if cards[0].rank == cards[1].rank
            && cards[1].rank == cards[2].rank
            && cards[2].rank == cards[3].rank
        {
            Some(PokerHand::FourOfAKind {
                quad: (cards[0], cards[1], cards[2], cards[3]),
                kicker: cards[4],
            })
        } else if cards[1].rank == cards[2].rank
            && cards[2].rank == cards[3].rank
            && cards[3].rank == cards[4].rank
        {
            Some(PokerHand::FourOfAKind {
                quad: (cards[1], cards[2], cards[3], cards[4]),
                kicker: cards[0],
            })
        } else {
            None
        }
    }

    fn straight_flush(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        if cards[0].suit == cards[1].suit
            && cards[1].suit == cards[2].suit
            && cards[2].suit == cards[3].suit
            && cards[3].suit == cards[4].suit
        {
            if cards[0].rank.next() == Some(cards[1].rank)
                && cards[1].rank.next() == Some(cards[2].rank)
                && cards[2].rank.next() == Some(cards[3].rank)
                && cards[3].rank.next() == Some(cards[4].rank)
            {
                Some(PokerHand::StraightFlush {
                    card1: cards[4],
                    card2: cards[3],
                    card3: cards[2],
                    card4: cards[1],
                    card5: cards[0],
                })
            } else {
                None
            }
        } else if cards[0].rank == Rank::Two
            && cards[1].rank == Rank::Three
            && cards[2].rank == Rank::Four
            && cards[3].rank == Rank::Five
            && cards[4].rank == Ace
        {
            if cards[0].suit == cards[1].suit
                && cards[1].suit == cards[2].suit
                && cards[2].suit == cards[3].suit
                && cards[3].suit == cards[4].suit
            {
                Some(PokerHand::StraightFlush {
                    card1: cards[3],
                    card2: cards[2],
                    card3: cards[1],
                    card4: cards[0],
                    card5: cards[4],
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn royal_flush(hand: &Hand) -> Option<PokerHand> {
        let mut cards = [hand.card1, hand.card2, hand.card3, hand.card4, hand.card5];
        cards.sort();
        if cards.iter().all(|card| card.suit == cards[0].suit)
            && cards[0].rank == Rank::Ten
            && cards[1].rank == Rank::Jack
            && cards[2].rank == Rank::Queen
            && cards[3].rank == Rank::King
            && cards[4].rank == Ace
        {
            Some(PokerHand::RoyalFlush)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_pair() {
        let hand = Hand {
            card1: Card::from_str("2S"),
            card2: Card::from_str("2D"),
            card3: Card::from_str("3S"),
            card4: Card::from_str("4S"),
            card5: Card::from_str("5S"),
        };
        let poker_hand = PokerHand::one_pair(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::OnePair {
                pair: (Card::from_str("2S"), Card::from_str("2D")),
                card3: Card::from_str("5S"),
                card4: Card::from_str("4S"),
                card5: Card::from_str("3S"),
            }
        );
    }

    #[test]
    fn test_two_pair() {
        let hand = Hand {
            card1: Card::from_str("2S"),
            card2: Card::from_str("2D"),
            card3: Card::from_str("3S"),
            card4: Card::from_str("3D"),
            card5: Card::from_str("5S"),
        };
        let poker_hand = PokerHand::two_pair(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::TwoPair {
                high_pair: (Card::from_str("3S"), Card::from_str("3D")),
                low_pair: (Card::from_str("2S"), Card::from_str("2D")),
                kicker: Card::from_str("5S"),
            }
        );
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = Hand {
            card1: Card::from_str("2S"),
            card2: Card::from_str("2D"),
            card3: Card::from_str("2C"),
            card4: Card::from_str("3D"),
            card5: Card::from_str("5S"),
        };
        let poker_hand = PokerHand::three_of_a_kind(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::ThreeOfAKind {
                triple: (
                    Card::from_str("2S"),
                    Card::from_str("2D"),
                    Card::from_str("2C")
                ),
                card4: Card::from_str("5S"),
                card5: Card::from_str("3D"),
            }
        );
    }

    #[test]
    fn test_straight() {
        let hand = Hand {
            card1: Card::from_str("2S"),
            card2: Card::from_str("3D"),
            card3: Card::from_str("4C"),
            card4: Card::from_str("5D"),
            card5: Card::from_str("6S"),
        };
        let poker_hand = PokerHand::straight(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::Straight {
                card1: Card::from_str("6S"),
                card2: Card::from_str("5D"),
                card3: Card::from_str("4C"),
                card4: Card::from_str("3D"),
                card5: Card::from_str("2S"),
            }
        );
    }

    #[test]
    fn test_flush() {
        let hand = Hand {
            card2: Card::from_str("3S"),
            card1: Card::from_str("2S"),
            card3: Card::from_str("5S"),
            card5: Card::from_str("TS"),
            card4: Card::from_str("8S"),
        };
        let poker_hand = PokerHand::flush(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::Flush {
                card1: Card::from_str("TS"),
                card2: Card::from_str("8S"),
                card3: Card::from_str("5S"),
                card4: Card::from_str("3S"),
                card5: Card::from_str("2S"),
            }
        );
    }

    #[test]
    fn test_full_house() {
        let hand = Hand {
            card2: Card::from_str("3S"),
            card1: Card::from_str("2S"),
            card3: Card::from_str("3D"),
            card5: Card::from_str("2D"),
            card4: Card::from_str("2C"),
        };
        let poker_hand = PokerHand::full_house(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::FullHouse {
                triple: (
                    Card::from_str("2S"),
                    Card::from_str("2D"),
                    Card::from_str("2C")
                ),
                pair: (Card::from_str("3S"), Card::from_str("3D")),
            }
        );

        let hand = Hand {
            card2: Card::from_str("3S"),
            card1: Card::from_str("2S"),
            card3: Card::from_str("3D"),
            card5: Card::from_str("2D"),
            card4: Card::from_str("3C"),
        };
        let poker_hand = PokerHand::full_house(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::FullHouse {
                triple: (
                    Card::from_str("3S"),
                    Card::from_str("3D"),
                    Card::from_str("3C")
                ),
                pair: (Card::from_str("2S"), Card::from_str("2D")),
            }
        );
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = Hand {
            card2: Card::from_str("3S"),
            card1: Card::from_str("2S"),
            card3: Card::from_str("3D"),
            card5: Card::from_str("3C"),
            card4: Card::from_str("3H"),
        };
        let poker_hand = PokerHand::four_of_a_kind(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::FourOfAKind {
                quad: (
                    Card::from_str("3S"),
                    Card::from_str("3D"),
                    Card::from_str("3C"),
                    Card::from_str("3H")
                ),
                kicker: Card::from_str("2S"),
            }
        );
    }

    #[test]
    fn test_straight_flush() {
        let hand = Hand {
            card2: Card::from_str("3S"),
            card1: Card::from_str("2S"),
            card3: Card::from_str("4S"),
            card5: Card::from_str("5S"),
            card4: Card::from_str("6S"),
        };
        let poker_hand = PokerHand::straight_flush(&hand).unwrap();
        assert_eq!(
            poker_hand,
            PokerHand::StraightFlush {
                card1: Card::from_str("6S"),
                card2: Card::from_str("5S"),
                card3: Card::from_str("4S"),
                card4: Card::from_str("3S"),
                card5: Card::from_str("2S"),
            }
        );
    }

    #[test]
    fn test_royal_flush() {
        let hand = Hand {
            card2: Card::from_str("JS"),
            card1: Card::from_str("TS"),
            card3: Card::from_str("QS"),
            card5: Card::from_str("KS"),
            card4: Card::from_str("AS"),
        };
        let poker_hand = PokerHand::royal_flush(&hand).unwrap();
        assert_eq!(poker_hand, PokerHand::RoyalFlush);
    }

    #[test]
    fn test_order() {
        let royal_flush = PokerHand::RoyalFlush;
        let straight_flush = PokerHand::StraightFlush {
            card1: Card::from_str("6S"),
            card2: Card::from_str("5S"),
            card3: Card::from_str("4S"),
            card4: Card::from_str("3S"),
            card5: Card::from_str("2S"),
        };
        assert!(royal_flush > straight_flush);

        let two_pair_1 = PokerHand::TwoPair {
            high_pair: (Card::from_str("3S"), Card::from_str("3D")),
            low_pair: (Card::from_str("2S"), Card::from_str("2D")),
            kicker: Card::from_str("5S"),
        };
        let two_pair_2 = PokerHand::TwoPair {
            high_pair: (Card::from_str("3S"), Card::from_str("3D")),
            low_pair: (Card::from_str("2S"), Card::from_str("2D")),
            kicker: Card::from_str("4S"),
        };
        assert!(two_pair_1 > two_pair_2);

        let two_pair_1 = PokerHand::TwoPair {
            high_pair: (Card::from_str("4S"), Card::from_str("4D")),
            low_pair: (Card::from_str("2S"), Card::from_str("2D")),
            kicker: Card::from_str("2S"),
        };
        let two_pair_2 = PokerHand::TwoPair {
            high_pair: (Card::from_str("4S"), Card::from_str("4D")),
            low_pair: (Card::from_str("3S"), Card::from_str("3D")),
            kicker: Card::from_str("AS"),
        };
        assert!(two_pair_1 < two_pair_2);
    }
}
