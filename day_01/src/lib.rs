use std::error::Error;

use regex::CaptureMatches;

pub fn extract_numbers(input: CaptureMatches) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut all_numbers = vec![];
    for (full_match, [_]) in input.map(|x| x.extract()) {
        all_numbers.push(full_match.parse::<usize>()?);
    }

    if all_numbers.is_empty() == true {
        return Err("no numbers found".into());
    }

    // at this point there is at least one number found
    // if there is ONLY one number, repeat the number
    // otherwise add the last number found
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

    Ok(res)
}

pub fn concatenate_numbers(input: Vec<usize>) -> usize {
    format!("{}{}", input.first().unwrap(), input.last().unwrap())
        .parse::<usize>()
        .unwrap()
}
