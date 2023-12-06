use std::collections::HashMap;

use day_03::Matrix;

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut schematic = Matrix::new(&input);

    println!("{schematic}");

    // part 1
    let mut result_part1: u32 = 0;
    for number in schematic.numbers() {
        if !schematic.number_has_any_adjacent_symbol(number.0, number.1) {
            continue;
        }
        result_part1 += number.0 as u32;
    }
    println!("\n=> Result for part 1 is {result_part1}");

    // part 2
    let mut gear_positions: HashMap<String, Vec<u16>> = HashMap::new();
    for number in schematic.numbers() {
        if let Some(pos) = schematic.number_has_special_adjacent_symbol('*', number.0, number.1) {
            let pos_key = format!("{}{}", pos.row(), pos.column());

            if let Some(existing) = gear_positions.get_mut(&pos_key) {
                existing.push(number.0);
            } else {
                gear_positions.insert(pos_key, vec![number.0]);
            }
        }
    }

    let mut result_part2: u32 = 0;
    for (_, number) in gear_positions {
        if number.len() == 2 {
            result_part2 += number[0] as u32 * number[1] as u32;
        }
    }

    println!("=> Result for part 2 is {result_part2}");
}
