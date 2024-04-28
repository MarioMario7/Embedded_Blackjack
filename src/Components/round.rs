use super::card::Card;

#[derive(PartialEq)]
#[derive(Copy, Clone)] 
pub enum Outcome {
    Ongoing,
    Win,
    Loss,
    Push,
    BJ_Win
    //maybe handle disconects
}

#[derive(PartialEq)]
#[derive(Copy, Clone)] 
pub struct Round 
{
    pub outcome: Outcome,
    pub bet: u16,
    pub cards: [Option<Card>; 53],
}

impl Round 
{
    pub fn new(bet: u16) -> Self {
        Round {
            outcome: Outcome::Ongoing,
            bet,
            cards: [None; 53],
        }
    }

    pub fn clear_cards(&mut self) {
        if self.outcome != Outcome::Ongoing 
        {
            self.cards = [Some(Card::placeholder()); 53];
        }
    }

    pub fn set_outcome(&mut self, outcome: Outcome) {
        self.outcome = outcome;
        self.clear_cards();
    }

    pub fn use_card(&mut self, card: Card) {
        for slot in self.cards.iter_mut() 
        {
            if slot.is_none() //adds at 1st empty position  
            {
                *slot = Some(card);
                break;
            }
        }
    }
}
