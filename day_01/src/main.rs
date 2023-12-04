use day_01::{
    extract_calibration_number, numbers_in_line, replace_number_strings, STRING_NUMBER_REGEX,
};

fn main() {
    let input_filename = "input4";
    let input = std::fs::read_to_string(input_filename).unwrap();
    let mut line_numbers: Vec<usize> = vec![];

    for line in input.lines() {
        let line_str = replace_number_strings(line);
        println! {"{:?}", line_str};
        match numbers_in_line(&line_str.to_string(), &STRING_NUMBER_REGEX) {
            Ok(numbers) => {
                if let Some(calibration_number) = extract_calibration_number(numbers) {
                    println! {"{line_str}: {calibration_number}"};
                    line_numbers.push(calibration_number);
                } else {
                    panic!("failed to extract calibration number")
                }
            }
            Err(e) => panic!("cannot extract numbers in line '{line_str}': {e}"),
        }
    }

    let result: usize = line_numbers.iter().map(|number| number).sum();

    println! {"\n=> The result is: {:?}", result};
}
