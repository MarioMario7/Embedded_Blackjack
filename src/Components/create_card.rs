use super::card::Card;
use super::SuitsRanks::{Suit, Rank};
use super::round::Round;

pub fn card_init(serial_input: &str, round: &Round) -> Card {
    let mut rank = Rank::Placeholder; // placeholder values
    let mut suit = Suit::NoSuit;

    let card_dict = [ // Strings will be replaced with their respective serials
        ("Ace Heart", (Rank::Ace, Suit::Heart)),
        ("Two Heart", (Rank::Two, Suit::Heart)),
        ("Three Heart", (Rank::Three, Suit::Heart)),
        ("Four Heart", (Rank::Four, Suit::Heart)),
        ("Five Heart", (Rank::Five, Suit::Heart)),
        ("Six Heart", (Rank::Six, Suit::Heart)),
        ("Seven Heart", (Rank::Seven, Suit::Heart)),
        ("Eight Heart", (Rank::Eight, Suit::Heart)),
        ("Nine Heart", (Rank::Nine, Suit::Heart)),
        ("Ten Heart", (Rank::Ten, Suit::Heart)),
        ("Jack Heart", (Rank::Jack, Suit::Heart)),
        ("Queen Heart", (Rank::Queen, Suit::Heart)),
        ("King Heart", (Rank::King, Suit::Heart)),
        ("Ace Diamond", (Rank::Ace, Suit::Diamond)),
        ("Two Diamond", (Rank::Two, Suit::Diamond)),
        ("Three Diamond", (Rank::Three, Suit::Diamond)),
        ("Four Diamond", (Rank::Four, Suit::Diamond)),
        ("Five Diamond", (Rank::Five, Suit::Diamond)),
        ("Six Diamond", (Rank::Six, Suit::Diamond)),
        ("Seven Diamond", (Rank::Seven, Suit::Diamond)),
        ("Eight Diamond", (Rank::Eight, Suit::Diamond)),
        ("Nine Diamond", (Rank::Nine, Suit::Diamond)),
        ("Ten Diamond", (Rank::Ten, Suit::Diamond)),
        ("Jack Diamond", (Rank::Jack, Suit::Diamond)),
        ("Queen Diamond", (Rank::Queen, Suit::Diamond)),
        ("King Diamond", (Rank::King, Suit::Diamond)),
        ("Ace Club", (Rank::Ace, Suit::Club)),
        ("Two Club", (Rank::Two, Suit::Club)),
        ("Three Club", (Rank::Three, Suit::Club)),
        ("Four Club", (Rank::Four, Suit::Club)),
        ("Five Club", (Rank::Five, Suit::Club)),
        ("Six Club", (Rank::Six, Suit::Club)),
        ("Seven Club", (Rank::Seven, Suit::Club)),
        ("Eight Club", (Rank::Eight, Suit::Club)),
        ("Nine Club", (Rank::Nine, Suit::Club)),
        ("Ten Club", (Rank::Ten, Suit::Club)),
        ("Jack Club", (Rank::Jack, Suit::Club)),
        ("Queen Club", (Rank::Queen, Suit::Club)),
        ("King Club", (Rank::King, Suit::Club)),
        ("Ace Spade", (Rank::Ace, Suit::Spade)),
        ("Two Spade", (Rank::Two, Suit::Spade)),
        ("Three Spade", (Rank::Three, Suit::Spade)),
        ("Four Spade", (Rank::Four, Suit::Spade)),
        ("Five Spade", (Rank::Five, Suit::Spade)),
        ("Six Spade", (Rank::Six, Suit::Spade)),
        ("Seven Spade", (Rank::Seven, Suit::Spade)),
        ("Eight Spade", (Rank::Eight, Suit::Spade)),
        ("Nine Spade", (Rank::Nine, Suit::Spade)),
        ("Ten Spade", (Rank::Ten, Suit::Spade)),
        ("Jack Spade", (Rank::Jack, Suit::Spade)),
        ("Queen Spade", (Rank::Queen, Suit::Spade)),
        ("King Spade", (Rank::King, Suit::Spade)),
        ("Cutting Card", (Rank::CuttingCard, Suit::NoSuit)),
        ("Placeholder", (Rank::Placeholder, Suit::NoSuit)),
    ];

    let mut found = false;

    // check validity of serial
    for &(serial_dict, (r, s)) in &card_dict 
    {
        if serial_input == serial_dict {
            rank = r;
            suit = s;
            found = true;
            break;
        }
    }

    // if we have the card in the round already, we must ignore it(misscan)
    let card_exists = round.cards.iter().any(|&c| 
        {
        if let Some(card) = c {
            card.rank == rank && card.suit == suit
        } else {
            false
        }
    });

    // we handle it by returning a placeholder card (TODO: check misscan in unit tests) 
    if card_exists {
       return Card::new(Rank::Placeholder,Suit::NoSuit);
    }



    if found {
        Card::new(rank,suit)
    } else {
        Card::new(Rank::Placeholder,Suit::NoSuit)
    }
}
