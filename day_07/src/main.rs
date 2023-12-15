use std::collections::HashMap;

use day_07::{CardHand, CardHandVec};

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut card_hands = CardHandVec(vec![]);
    let mut bids: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        let card_hand =
            CardHand::from_string(line.split_whitespace().into_iter().next().unwrap()).unwrap();
        let bid: usize = line
            .split_whitespace()
            .into_iter()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        bids.insert((&card_hand).to_string(), bid);
        card_hands.push(card_hand);
    }
    card_hands.sort();

    let mut result = 0;
    for (index, hand) in card_hands.iter().enumerate() {
        let rank = index + 1;
        let bid = bids.get(&hand.to_string()).unwrap();

        println! {"{hand} ({:?}) has rank {rank} and bid {bid}", hand.evaluate_type()};
        result += rank * bid;
    }

    println!("=> Result is {result}");
}
