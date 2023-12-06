use std::collections::HashSet;

use regex::Regex;

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let re = Regex::new(
        r"Card\s+(?P<card_id>\d+):\s+(?P<wining_numbers>(\d+|\s+)+)\|\s+(?P<my_numbers>(\d+|\s+)+)$",
    )
    .unwrap();

    let mut result = 0;
    for line in input.lines() {
        for capture in re.captures_iter(line) {
            // let card_id = capture.name("card_id").unwrap().as_str();
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

            println! {"winning: {:?}", winning_numbers};
            println! {"mine: {:?}", my_numbers};

            let winning_set: HashSet<u16> = HashSet::from_iter(winning_numbers.iter().cloned());
            let my_set: HashSet<u16> = HashSet::from_iter(my_numbers.iter().cloned());

            let my_wins = winning_set.intersection(&my_set);
            let points = my_wins.clone().count();
            if points == 0 {
                println!("==================================");
                continue;
            }

            let base: u32 = 2;
            let res = base.pow(points as u32 - 1);
            result += res;

            println! {"matches: {:?}\npoints: {points} => {res}", my_wins};

            println!("==================================");
        }
    }

    println!("=> Result is: {result}")
}
