use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Eq)]
pub struct Card(pub char);

// const CARD_VALUES: &str = "AKQJT98765432"; // part 1
const CARD_VALUES: &str = "AKQT98765432J"; // part 2

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

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn hand_type(hand: &CardHand) -> HandType {
        let mut card_counts = hand.card_count();

        // at this point we loose the ability to map counts to Cards
        card_counts.sort();
        card_counts.reverse();

        // possible card_counts for HandType if they are sorted
        //
        // HighCard:    1,1,1,1,1,0,0,...
        // OnePair:     2,1,1,1,0,0,0,...
        // TwoPair:     2,2,1,0,0,0,0,...
        // ThreeOfKind: 3,1,1,0,0,0,0,...
        // FullHouse:   3,2,0,0,0,0,0,...
        // FourOfKind:  4,1,0,0,0,0,0,...
        // FiveOfKind:  5,0,0,0,0,0,0,...
        match &card_counts[0..5] {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1, 0] => HandType::OnePair,
            [2, 2, 1, 0, 0] => HandType::TwoPair,
            [3, 1, 1, 0, 0] => HandType::ThreeOfKind,
            [3, 2, 0, 0, 0] => HandType::FullHouse,
            [4, 1, 0, 0, 0] => HandType::FourOfKind,
            [5, 0, 0, 0, 0] => HandType::FiveOfKind,
            _ => panic!("impossible pattern"),
        }
    }

    fn project_joker(joker_count: usize, input: HandType) -> HandType {
        let mut projections: Vec<(usize, HandType, HandType)> = vec![];
        projections.push((1, HandType::HighCard, HandType::OnePair));
        projections.push((1, HandType::OnePair, HandType::ThreeOfKind));
        projections.push((1, HandType::TwoPair, HandType::FullHouse));
        projections.push((1, HandType::ThreeOfKind, HandType::FourOfKind));
        projections.push((1, HandType::FourOfKind, HandType::FiveOfKind));
        projections.push((2, HandType::OnePair, HandType::ThreeOfKind));
        projections.push((2, HandType::TwoPair, HandType::FourOfKind));
        projections.push((2, HandType::FullHouse, HandType::FiveOfKind));
        projections.push((3, HandType::ThreeOfKind, HandType::FourOfKind));
        projections.push((3, HandType::FullHouse, HandType::FiveOfKind));

        if let Some(proj) = projections
            .iter()
            .find(|x| x.0 == joker_count && x.1 == input)
        {
            // println! {"project {:?} with {:?} jokers to {:?}", proj.1, proj.0, proj.2};
            return proj.2;
        }

        input
    }

    pub fn evaluate_type(&self) -> HandType {
        let card_type = CardHand::hand_type(&self);

        // no joker? this is the best we can do
        if !self.to_string().contains("J") {
            return card_type;
        }

        // Projecting jokers is just applying a projection map which is derived
        // from the cases below.
        //
        // 1 Joker
        //  - if HighCard -> OnePair
        //  - if OnePair -> ThreeOfKind; could also be TwoPair but ThreeOfKind is better
        //  - if TwoPair -> FullHouse
        //  - if ThreeOfKind -> FourOfKind; could also be FullHouse but FourOfKind is better
        //  - if FullHouse -> Cannot happen!
        //  - if FourOfKind -> FiveOfKind
        //  - if FiveOfKind -> Cannot happen! There must be exactly one joker!
        // 2 Joker
        //  - if HighCard -> Cannot happen! There must be exactly two jokers!
        //  - if OnePair -> ThreeOfKind
        //  - if TwoPair -> FourOfKind; could also be FullHouse but FourOfKind is better
        //  - if ThreeOfKind -> Cannot happen! The jokers already make up a pair.
        //  - if FullHouse -> FiveOfKind
        //  - if FourOfKind -> Cannot happen!
        //  - if FiveOfKind -> Cannot happen!
        // 3 Joker
        //  - if HighCard -> Cannot happen!
        //  - if OnePair -> Cannot happen!
        //  - if TwoPair -> Cannot happen!
        //  - if ThreeOfKind -> FourOfKind
        //  - if FullHouse -> FiveOfKind
        //  - if FourOfKind -> Cannot happen!
        //  - if FiveOfKind -> Cannot happen!
        // 4 Joker -> FiveOfKind
        // 5 joker -> FiveOfKind
        let jokers: Vec<char> = self.to_string().chars().filter(|c| *c == 'J').collect();
        match jokers.len() {
            1 | 2 | 3 => CardHand::project_joker(jokers.len(), card_type),
            4 | 5 => HandType::FiveOfKind,
            _ => panic!("illegal CardHand"),
        }
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
