use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Eq)]
pub struct Card(pub char);

const CARD_VALUES: &str = "AKQJT98765432";

impl Card {
    pub fn new(value: char) -> Result<Card, String> {
        if let Some(_) = CARD_VALUES.find(value.to_ascii_uppercase()) {
            return Ok(Card(value.to_ascii_uppercase()));
        }

        Err("invalid card value".to_string())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let order: String = CARD_VALUES.chars().rev().collect();

        let self_index = order.find(self.0);
        let other_index = order.find(other.0);

        self_index.partial_cmp(&other_index)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Deref for Card {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Card {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Eq)]
pub struct CardHand(pub Vec<Card>);

impl CardHand {
    pub fn from_string(input: &str) -> Result<CardHand, String> {
        return input.chars().map(|c| Card::new(c)).collect();
    }

    fn card_count(&self) -> Vec<u8> {
        let mut counts: Vec<u8> = vec![0; CARD_VALUES.len()];

        for card in &self.0 {
            let card_index = CARD_VALUES.find(card.0).unwrap();
            counts[card_index] += 1;
        }

        counts
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }

    pub fn evaluate_type(&self) -> HandType {
        let mut card_counts = self.card_count();

        // possible card_counts for HandType if they are sorted
        //
        // HighCard:    1,1,1,1,1,0,0,...
        // OnePair:     2,1,1,1,0,0,0,...
        // TwoPair:     2,2,1,0,0,0,0,...
        // ThreeOfKind: 3,1,1,0,0,0,0,...
        // FullHouse:   3,2,0,0,0,0,0,...
        // FourOfKind:  4,1,0,0,0,0,0,...
        // FiveOfKind:  5,0,0,0,0,0,0,...

        // at this point we loose the ability to map counts to Cards
        card_counts.sort();
        card_counts.reverse();

        match &card_counts[0..5] {
            [1, 1, 1, 1, 1] => return HandType::HighCard,
            [2, 1, 1, 1, 0] => return HandType::OnePair,
            [2, 2, 1, 0, 0] => return HandType::TwoPair,
            [3, 1, 1, 0, 0] => return HandType::ThreeOfKind,
            [3, 2, 0, 0, 0] => return HandType::FullHouse,
            [4, 1, 0, 0, 0] => return HandType::FourOfKind,
            [5, 0, 0, 0, 0] => return HandType::FiveOfKind,
            _ => panic!("impossible pattern"),
        };
    }
}

impl FromIterator<Card> for CardHand {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut card_hand = CardHand(vec![]);

        for c in iter {
            card_hand.push(c)
        }

        card_hand
    }
}

impl Deref for CardHand {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CardHand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.evaluate_type() == other.evaluate_type()
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = self.evaluate_type() as u8;
        let other_value = other.evaluate_type() as u8;

        // if both hands are of equal type (e.g. TwoPair)
        // we need to compare card by card
        if self_value == other_value {
            for index in 0..self.0.len() {
                let self_card = &self.0[index];
                let other_card = &other.0[index];

                if self_card == other_card {
                    continue;
                }

                return self_card.partial_cmp(&other_card);
            }
        }

        self_value.partial_cmp(&other_value)
    }
}

impl Debug for CardHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\n\t{} - {:?}", &self, self.evaluate_type(),))?;
        Ok(())
    }
}

impl Display for CardHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: Vec<String> = self.0.iter().map(|card| card.to_string()).collect();
        f.write_str(&c.join(""))
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_val = self.evaluate_type() as u8;
        let other_val = other.evaluate_type() as u8;

        self_val
            .partial_cmp(&other_val)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

#[derive(Debug)]
pub struct CardHandVec(pub Vec<CardHand>);

impl Deref for CardHandVec {
    type Target = Vec<CardHand>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CardHandVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
