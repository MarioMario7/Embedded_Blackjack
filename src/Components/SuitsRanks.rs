
#[derive(PartialEq)]
#[derive(Copy, Clone)] // Implement Copy and Clone
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
    NoSuit
}

#[derive(PartialEq)]
#[derive(Copy, Clone)] // Implement Copy and Clone
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    CuttingCard,
    Placeholder
}