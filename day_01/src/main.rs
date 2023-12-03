use day_01::{concatenate_numbers, extract_numbers};
use regex::Regex;

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();
    let mut line_numbers: Vec<usize> = vec![];

    let re = Regex::new(r"(\d)").unwrap();

    for line in input.lines() {
        match extract_numbers(re.captures_iter(line)) {
            Ok(numbers) => {
                println! {"{}: {:?}", line, numbers};
                line_numbers.push(concatenate_numbers(numbers));
            }
            Err(e) => {
                eprintln!("invalid line '{line}': {e}");
                continue;
            }
        }
    }

    let result: usize = line_numbers.iter().map(|number| number).sum();

    println! {"\n=> The result is: {:?}", result};
}
