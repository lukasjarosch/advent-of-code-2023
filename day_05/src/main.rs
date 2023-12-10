use std::{collections::HashMap, u64};

use day_05::{parse_map_line, resolve_seed, Mapping};

fn main() {
    let input_filename = "input2";
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut seed_numbers: Vec<u64> = vec![];
    let mut mappings: Vec<Vec<Mapping>> = vec![];
    let mut line_map_buffer: Vec<Mapping> = vec![];
    for (line_number, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        if let Some(seeds) = line.split_once("seeds:") {
            seed_numbers = seeds
                .1
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            continue;
        }

        // if we encounter a string with 'map' we know that a new mapping starts
        // and if the mapping buffer is not empty, we need to merge all mappings into one map and
        // shove it into the mappings vector
        if line.contains("map:") {
            println! {"{:?}", line};
            if line_map_buffer.len() > 0 {
                mappings.push(line_map_buffer.clone());
                line_map_buffer.clear();
            }

            continue;
        }

        // at the end of the input, merge the map buffer once more
        if line_number == input.lines().count() - 1 {
            mappings.push(line_map_buffer.clone());
            line_map_buffer.clear();
        }

        let line_map = parse_map_line(line);
        match line_map {
            None => {
                panic!("broken, lol")
            }
            Some(map) => {
                line_map_buffer.push(map);
            }
        }
    }

    let mut location_numbers: Vec<u64> = vec![];
    for seed in seed_numbers {
        location_numbers.push(resolve_seed(&mappings, seed));
    }

    println! {"=> Smallest location value is: {:?}", location_numbers.iter().min().unwrap()};
}
