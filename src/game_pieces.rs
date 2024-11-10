use std::{cmp, fmt};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Suit {
    HEART,
    CLUB,
    DIAMOND,
    SPADE,
}

// pub const TRUMP_SUIT: Suit = Suit::HEART;
pub const SUITS: [Suit; 4] = [Suit::HEART, Suit::CLUB, Suit::DIAMOND, Suit::SPADE];

impl Suit {
    fn symbol(&self) -> char {
        match *self {
            Self::HEART => '♥',
            Self::CLUB => '♣',
            Self::DIAMOND => '♦',
            Self::SPADE => '♠',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Value {
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

pub const VALUES: [Value; 13] = [
    Value::TWO,
    Value::THREE,
    Value::FOUR,
    Value::FIVE,
    Value::SIX,
    Value::SEVEN,
    Value::EIGHT,
    Value::NINE,
    Value::TEN,
    Value::JACK,
    Value::QUEEN,
    Value::KING,
    Value::ACE,
];

impl Value {
    fn symbol(&self) -> char {
        match *self {
            Self::TWO => '2',
            Self::THREE => '3',
            Self::FOUR => '4',
            Self::FIVE => '5',
            Self::SIX => '6',
            Self::SEVEN => '7',
            Self::EIGHT => '8',
            Self::NINE => '9',
            Self::TEN => 'T',
            Self::JACK => 'J',
            Self::QUEEN => 'Q',
            Self::KING => 'K',
            Self::ACE => 'A',
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Card {
    suit: Suit,
    value: Value,
    trump: bool,
}

impl Card {
    pub fn new(value: Value, suit: Suit, trump: bool) -> Self {
        Self { suit, value, trump }
    }

    fn update_trump(mut self, trump_suit: Suit) -> Self {
        if self.suit == trump_suit {
            self.trump = true
        }

        self
    }

    pub fn is_trump(&self) -> bool {
        self.trump
    }

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn value(&self) -> Value {
        self.value
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit.symbol(), self.value.symbol())?;
        Ok(())
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit.symbol(), self.value.symbol())?;
        Ok(())
    }
}

impl cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self.trump, other.trump) {
            (true, true) => self.value.partial_cmp(&other.value),
            (true, false) => Some(cmp::Ordering::Greater),
            (false, true) => Some(cmp::Ordering::Less),
            (false, false) => self.value.partial_cmp(&other.value),
        }
    }
}

impl cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.trump, other.trump) {
            (true, true) => self.value.cmp(&other.value),
            (true, false) => cmp::Ordering::Greater,
            (false, true) => cmp::Ordering::Less,
            (false, false) => self.value.cmp(&other.value),
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
    final_card_value: Value,
    trump_suit: Suit
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::<Card>::new();
        for value in VALUES {
            for suit in SUITS {
                cards.push(Card::new(value, suit, false));
            }
        }

        cards.shuffle(&mut thread_rng());
        
        let final_card_value = cards[0].value;
        let trump_suit = cards[0].suit;

        cards = cards
            .into_iter()
            .map(|c| c.update_trump(trump_suit))
            .collect();

        Self { cards, final_card_value, trump_suit }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn draw_cards(&mut self, n: u8) -> Vec<Card> {
        let mut cards = Vec::<Card>::new();

        for _ in 0..n {
            self.draw_card().map(|c| cards.push(c));
        }

        cards
    }

    pub fn trump_suit(&self) -> Suit {
        self.trump_suit
    }
}
