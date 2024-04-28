use super::card::Card;
use super::SuitsRanks::{Suit, Rank};
use super::round::Round;

pub fn addCard(card: &Card, round: &Round, target: &Player) -> (){

    target.score += card.value;

    round.use_card(card);

    if card.rank == Rank::Ace 
    {
        target.ace_score += card.value_ace_upper;
    }
}
