// Hold'em hand rankings
// Code evolved from a preliminary version uploaded to:
// https://rosettacode.org/wiki/Poker_hand_analyser
// All versions (including the Rust implementation on the site) by rustkel

// Version 0.2
// Jokers have been removed
#![allow(dead_code)]

#[derive(Debug)]
pub enum Hand {
    Invalid = -1,
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

pub fn poker_hand(cards: &str) -> Hand {
    let mut suits = vec![0u8; 4];
    let mut faces = vec![0u8; 15];
    let mut hand = vec![];

    for card in cards.chars() {
        if card == ' ' {
            continue;
        }
        let values = get_card_value(card);
        if values.0 < 14 && hand.contains(&values) {
            return Hand::Invalid;
        }
        hand.push(values);
        faces[values.0 as usize] += 1;
        suits[values.1 as usize] += 1;
    }
    if hand.len() != 5 {
        return Hand::Invalid;
    }
    faces[13] = faces[0]; //add ace-high count

    //count suits
    let mut colors = suits.into_iter().filter(|&x| x > 0).collect::<Vec<_>>();
    colors.sort_unstable();
    let is_flush = colors[0] == 5;

    //straight
    let mut is_straight = false;
    let mut straight_cards = 0;
    let mut highest_card = 13;
    for i in (0..14).rev() {
        if faces[i] == 0 {
            straight_cards = 0;
            highest_card = i - 1;
        } else {
            straight_cards += 1;
            if straight_cards == 5 {
                is_straight = true;
                break;
            }
        }
    }
    //count values
    let mut values = faces
        .into_iter()
        .enumerate()
        .take(14)
        .filter(|&x| x.1 > 0)
        .collect::<Vec<_>>();
    //sort by quantity, then by value, high to low
    values.sort_unstable_by(|a, b| {
        if b.1 == a.1 {
            (b.0).cmp(&a.0)
        } else {
            (b.1).cmp(&a.1)
        }
    });
    let first_group = values[0].1;
    let second_group = if values.len() > 1 { values[1].1 } else { 0 };
    match (is_flush, is_straight, first_group, second_group) {
        (true, true, _, _) => {
            if highest_card == 13 {
                Hand::RoyalFlush
            } else {
                Hand::StraightFlush
            }
        }
        (_, _, 4, _) => Hand::FourOfAKind,
        (_, _, 3, 2) => Hand::FullHouse,
        (true, _, _, _) => Hand::Flush,
        (_, true, _, _) => Hand::Straight,
        (_, _, 3, _) => Hand::ThreeOfAKind,
        (_, _, 2, 2) => Hand::TwoPair,
        (_, _, 2, _) => Hand::Pair,
        _ => Hand::HighCard,
    }
}

fn get_card_value(card: char) -> (u8, u8) {
    // transform glyph to face + suit, zero-indexed
    let base = card as u32 - 0x1F0A1;
    let suit = base / 16;
    let mut face = base % 16;
    if face > 13 {
        panic!("Jokers not allowed")
    }
    if face > 11 {
        face -= 1;
    } // Unicode has a Knight that we do not want
    (face as u8, suit as u8)
}
