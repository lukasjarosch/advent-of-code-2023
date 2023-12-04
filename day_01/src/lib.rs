use std::{error::Error, str::from_utf8, vec};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref NUMBER_REGEX: Regex = Regex::new(r"(\d)").unwrap();
    pub static ref STRING_NUMBER_REGEX: Regex =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)|(\d)").unwrap();
}

fn decode_string_number(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => "0",
    }
}

pub fn replace_number_strings(input: &str) -> String {
    let number_strings = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut line = String::from(input);

    let mut positions: Vec<(usize, &str)> = vec![];
    for num_string in number_strings {
        for (offset, _) in input.match_indices(num_string) {
            positions.push((offset, decode_string_number(num_string)));
        }
    }

    for (offset, digit) in positions {
        line.replace_range(offset..offset + 1, digit);
    }

    line
}

pub fn numbers_in_line(line: &str, re: &Regex) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut all_numbers = vec![];

    for (full_match, [_]) in re.captures_iter(line).map(|x| x.extract()) {
        all_numbers.push(full_match.parse::<usize>()?);
    }

    if all_numbers.is_empty() == true {
        return Err("no numbers found".into());
    }

    Ok(all_numbers)
}

pub fn extract_calibration_number(all_numbers: Vec<usize>) -> Option<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(2);
    match all_numbers.len() {
        1 => {
            res.push(*all_numbers.first().unwrap());
            res.push(*all_numbers.first().unwrap());
        }
        _ => {
            res.push(*all_numbers.first().unwrap());
            res.push(*all_numbers.last().unwrap());
        }
    }

    Some(
        format!(
            "{}{}",
            all_numbers.first().unwrap(),
            all_numbers.last().unwrap()
        )
        .parse::<usize>()
        .unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_calibration_number() {
        assert_eq!(extract_calibration_number(vec![1, 2, 3]).unwrap(), 13);
        assert_eq!(extract_calibration_number(vec![3, 3]).unwrap(), 33);
        assert_eq!(extract_calibration_number(vec![8]).unwrap(), 88);
        assert_eq!(
            extract_calibration_number(vec![4, 4, 4, 4, 4, 4, 4, 2]).unwrap(),
            42
        );
    }

    #[test]
    fn test_string_numbers_in_line() {
        assert_eq!(
            numbers_in_line("one1two3", &STRING_NUMBER_REGEX).unwrap(),
            vec![1, 1, 2, 3]
        );
        assert_eq!(
            numbers_in_line("one9xmhvzklmzffive1kcsixmnsbm2", &STRING_NUMBER_REGEX).unwrap(),
            vec![1, 9, 5, 1, 6, 2]
        );
        assert_eq!(
            numbers_in_line("1dgschj", &STRING_NUMBER_REGEX).unwrap(),
            vec![1]
        );
        assert_eq!(
            numbers_in_line(
                "vfh4zteightkvbpps4rxhlnctjztjfvdvdxfk",
                &STRING_NUMBER_REGEX
            )
            .unwrap(),
            vec![4, 8, 4]
        );
    }
}
