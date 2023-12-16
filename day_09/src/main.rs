use std::isize;

use day_09::{mutate_until_zero, predict_values_left, predict_values_right};

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let all_histories: Vec<Vec<isize>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    // part 1
    let mut history_predictions: Vec<isize> = vec![];
    for history in all_histories.iter() {
        let permutations = mutate_until_zero(&history);
        let predictions = predict_values_right(&permutations);
        history_predictions.push(predictions.first().unwrap().clone());
    }
    let result: isize = history_predictions.iter().sum();
    println!("=> Result is {result}");

    // part 2
    let mut history_predictions: Vec<isize> = vec![];
    for history in all_histories.iter() {
        let permutations = mutate_until_zero(&history);
        let predictions = predict_values_left(&permutations);
        history_predictions.push(predictions.first().unwrap().clone());
    }
    let result: isize = history_predictions.iter().sum();
    println!("=> Result is {result}");
}
