
use super::SuitsRanks::{Suit, Rank};

#[derive(Copy)]
#[derive(Clone)]
pub struct Card {
    pub value: u16,
    pub rank: Rank,
    pub suit: Suit,
    pub value_ace_upper: Option<u16>, // 2nd value for the ace (ace is 1/11)
}

impl Card {
    //regular card
    pub fn new(rank: Rank, suit: Suit) -> Self {
        let value = match rank {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::CuttingCard => 0,
            Rank::Placeholder => 0
        };

        let value_ace_upper = None;

        if rank == Rank::Ace
        {
            let value_ace_upper = Some(11);
        }

        Card { value, rank, suit, value_ace_upper }
    }

    //cutting card - round is over if scanned
    pub fn new_cutting_card() -> Self {
        Card {
            value: 0,
            rank: Rank::CuttingCard,
            suit: Suit::NoSuit, 
            value_ace_upper: None
        }
    }

    pub fn placeholder() -> Self {
        Card {
            value: 0, 
            rank: Rank::Placeholder,
            suit: Suit::NoSuit, 
            value_ace_upper: None
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.rank == other.rank && self.suit == other.suit
    }
}