use crate::card::Card;
use crate::hand::Hand;

mod card;
mod hand;
mod poker_hand;
mod rank;
mod suit;

fn main() {
    let input = include_str!("../p054_poker.txt");
    let p1_wins = input
        .lines()
        .filter(|line| {
            let cards: Vec<&str> = line.split_whitespace().collect();
            let p1_hand = Hand {
                card1: Card::from_str(cards[0]),
                card2: Card::from_str(cards[1]),
                card3: Card::from_str(cards[2]),
                card4: Card::from_str(cards[3]),
                card5: Card::from_str(cards[4]),
            };
            let p2_hand = Hand {
                card1: Card::from_str(cards[5]),
                card2: Card::from_str(cards[6]),
                card3: Card::from_str(cards[7]),
                card4: Card::from_str(cards[8]),
                card5: Card::from_str(cards[9]),
            };
            p1_hand > p2_hand
        })
        .count();
    println!("Player 1 wins {} hands", p1_wins);
}
