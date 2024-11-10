mod game_pieces;

use std::collections::HashSet;

use game_pieces::{Card, Deck, Suit, Value};


fn main() {
    let mut results = Vec::<(String, f32, Suit)>::new();

    for _ in 0..30 {
        let mut deck = Deck::new();
        let mut hand = deck.draw_cards(6);
        hand.sort();
        results.push((format!("{:?}", hand), score_hand(&hand), deck.trump_suit()));
    }

    results.sort_by(|(_, a, _), (_, b, _)| a.partial_cmp(b).unwrap());

    for (hand, score, trump) in results.iter() {
        println!("{} ({}): {}", hand, trump, score)
    }

}

fn score_hand(hand: &Vec<Card>) -> f32 {
    let trump_coef = 13;
    let distinct_val_coef: f32 = 0.4;

    let n: f32 = hand.len() as f32;

    let mut score: f32 = hand
        .iter()
        .map(|c| {
                match c.is_trump() {
                    true => (c.value() as u32) + 1 + (Value::ACE as u32 * 3),
                    false => 3 * (c.value() as u32),
                }
        })
        .sum::<u32>() as f32;

    let u: f32 = hand.iter().map(|c| c.value()).collect::<HashSet<_>>().len() as f32;
    score *= (((n + 1.0) - u) / n) * distinct_val_coef + (1.0 - (distinct_val_coef / 2.0));

    score
}
