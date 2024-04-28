use super::card::Card;
use super::SuitsRanks::{Suit, Rank};
use super::round::Round;

pub struct Player {
    pub balance: u16,
    pub nr_wins: u16,
    pub score:u16,
    pub ace_score:u16   
}

impl Player {
    pub fn new() -> Self {
        Self {
            balance: 0,
            nr_wins: 0,
            score: 0,
            ace_score: 0
        }
    }

    pub fn reset(&mut self) 
    {
        self.balance = 0;
        self.nr_wins = 0;
        self.score = 0;
        self.ace_score = 0;
    }

    pub fn add_card(&mut self,card: &Card, round: &mut Round) 
    {

        self.score += card.value;
        
    
        round.use_card(*card); // add card to card list for curr round
    
        if card.rank == Rank::Ace 
        {
            self.ace_score = self.score;
            self.ace_score += 10;
        }
    }
}
