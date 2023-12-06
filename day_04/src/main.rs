use std::collections::HashSet;

use day_04::Card;
use regex::Regex;

fn main() {
    let input_filename = "input1";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let re = Regex::new(
        r"Card\s+(?P<card_id>\d+):\s+(?P<wining_numbers>(\d+|\s+)+)\|\s+(?P<my_numbers>(\d+|\s+)+)$",
    )
    .unwrap();

    let mut cards: Vec<Card> = vec![];

    let mut result = 0;
    for line in input.lines() {
        for capture in re.captures_iter(line) {
            let card_id: u16 = capture.name("card_id").unwrap().as_str().parse().unwrap();
            let winning_numbers_string = capture.name("wining_numbers").unwrap().as_str();
            let my_numbers_string = capture.name("my_numbers").unwrap().as_str();

            let winning_numbers: Vec<u16> = winning_numbers_string
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let my_numbers: Vec<u16> = my_numbers_string
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            let card = Card::new(card_id, winning_numbers, my_numbers);
            result += card.points();
            cards.push(card);
        }
    }

    println!("=> Result is: {result}")
}
