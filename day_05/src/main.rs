use rayon::prelude::*;
use std::{ops::Range, u64};

use day_05::{
    get_mapping_by_dest, get_min_dest_mapping, min_value, resolve_seed, solution2, Mapping,
};

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

        let line_map = Mapping::new_from_string(line);
        match line_map {
            None => {
                panic!("broken, lol")
            }
            Some(map) => {
                line_map_buffer.push(map);
            }
        }
    }

    // Part 1
    let mut location_numbers: Vec<u64> = vec![];
    for seed in seed_numbers.iter() {
        location_numbers.push(resolve_seed(&mappings, *seed));
    }
    println! {"=> Smallest location value is: {:?}", location_numbers.iter().min().unwrap()};

    // For part 2, seed numbers need to be converted to ranges
    // let seed_ranges: Vec<Range<_>> = seed_numbers
    //     .chunks(2)
    //     .into_iter()
    //     .map(|pair| pair[0]..pair[0] + pair[1])
    //     .collect();
    //
    // println! {"SEEDS: {:?}", seed_ranges};
    //
    // for mapping in mappings.iter() {
    //     for map in mapping {
    //         println! {"[{:?}]-[{:?}]", map.source_range(), map.destination_range()};
    //         // if map.destination_range().contains(&location) {
    //         //     println! {"{:?}", map.source_range()};
    //         //     break;
    //         // }
    //     }
    // }
    //
    //
    println! {"{:?}", solution2(&input)};

    // let mut seed_numbers_part2: Vec<u64> = vec![];
    // for seed_pair in seed_numbers.chunks(2) {
    //     println! {"===== INITIALIZING PAIR {:?} ====", seed_pair};
    //     let range = seed_pair[0]..seed_pair[0] + seed_pair[1];
    //     let mut tmp: Vec<u64> = range.into_par_iter().map(|num| num).collect();
    //     seed_numbers_part2.append(&mut tmp);
    //     break;
    // }
    //
    // // BRUTE FORCE: CPU goes brrrrr
    // let location_numbers_part2: Vec<u64> = seed_numbers_part2
    //     .par_iter()
    //     .enumerate()
    //     .map(|(idx, num)| {
    //         println! {"===== RESOLVING {:>5.2}% ({} left) ====",  (idx as f64 / seed_numbers_part2.len() as f64) * 100.0, format!("{:>12}", seed_numbers_part2.len() - idx)};
    //         resolve_seed(&mappings, *num)
    //     })
    //     .collect();
    //
    // println! {"=> Smallest location value is: {:?}", location_numbers_part2.par_iter().min().unwrap()};

    // println! {"Smallest location mapping is {:?}", get_min_dest_mapping(mappings.last().unwrap())};
    //
    // for mapping in mappings.iter().rev() {
    //     let map = get_min_dest_mapping(mapping);
    //     println! {"{:?}", map};
    // }
}
