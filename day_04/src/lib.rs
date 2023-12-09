use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

#[derive(Debug)]
pub struct Card {
    pub number: u16,
    pub matching_numbers: Vec<u16>,
}

impl Card {
    pub fn new(number: u16, winning_numbers: Vec<u16>, my_numbers: Vec<u16>) -> Card {
        let winning_set: HashSet<u16> = HashSet::from_iter(winning_numbers.iter().cloned());
        let my_set: HashSet<u16> = HashSet::from_iter(my_numbers.iter().cloned());
        let my_wins = winning_set
            .intersection(&my_set.clone())
            .map(|x| *x)
            .collect();

        Card {
            number,
            matching_numbers: my_wins,
        }
    }

    pub fn points(&self) -> u32 {
        if self.matching_numbers.len() == 0 {
            return 0;
        }
        let base: u32 = 2;
        base.pow(self.matching_numbers.len() as u32 - 1)
    }

    pub fn winning_card_numbers(&self) -> Option<Range<u16>> {
        if self.matching_numbers.len() == 0 {
            return None;
        }

        Some((self.number + 1)..(self.number + 1 + self.matching_numbers.len() as u16))
    }
}

pub fn calculate_card_count(
    original_cards: &HashMap<u16, Card>,
    card: &Card,
    mut count: usize,
) -> usize {
    if let Some(cards_numbers_won) = card.winning_card_numbers() {
        println! {"-> Card #{} won {:?}", card.number, cards_numbers_won};
        let cards_won: Vec<&Card> = cards_numbers_won
            .map(|number| original_cards.get(&number).unwrap())
            .collect();

        for card_copy in cards_won {
            count = calculate_card_count(original_cards, card_copy, count + 1);
        }
    }

    count
}
