use std::{collections::HashMap, error::Error, ops::Range, u64};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

#[derive(Clone, Copy, Debug)]
pub struct Mapping {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl Mapping {
    pub fn new(source_start: u64, destination_start: u64, length: u64) -> Mapping {
        Mapping {
            source_start,
            destination_start,
            length,
        }
    }

    pub fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.length - 1
    }

    pub fn destination_range(&self) -> Range<u64> {
        self.destination_start..self.destination_start + self.length - 1
    }

    pub fn new_from_string(line: &str) -> Option<Mapping> {
        let re =
            Regex::new(r"(?P<dst_range_start>\d+)\s*(?P<src_range_start>\d+)\s*(?P<length>\d+)")
                .unwrap();

        let capt = re.captures(line)?;
        let dst_range_start: u64 = capt.name("dst_range_start")?.as_str().parse().unwrap();
        let src_range_start: u64 = capt.name("src_range_start")?.as_str().parse().unwrap();
        let length: u64 = capt.name("length")?.as_str().parse().unwrap();

        Some(Mapping::new(src_range_start, dst_range_start, length))
    }

    pub fn map_source_to_destination(&self, key: u64) -> Option<u64> {
        // source_start: 10
        // destination_start: 25
        // length: 5
        // source_range: 10, 11, 12, 13, 14
        // destination_range: 25, 26, 27, 28, 29
        // key: 12
        // offset: 2
        // mapping: destination_range + offset -> 25 + 2 = 27
        if key >= self.source_start && key < self.source_start + self.length {
            let offset = key - self.source_start;
            return Some(self.destination_start + offset);
        }
        None
    }

    pub fn has_destination_value(&self, number: u64) -> bool {
        number >= self.destination_start && number < self.destination_start + self.length
    }
}

pub fn min_value(mappings: &Vec<Mapping>) -> u64 {
    let dest_start: Vec<u64> = mappings.par_iter().map(|m| m.destination_start).collect();
    dest_start.par_iter().min().unwrap().clone()
}

pub fn get_mapping_by_dest(mappings: &Vec<Mapping>, dst_value: u64) -> Option<&Mapping> {
    for map in mappings {
        if map.has_destination_value(dst_value) {
            return Some(map);
        }
    }

    None
}

pub fn get_min_dest_mapping(mappings: &Vec<Mapping>) -> &Mapping {
    get_mapping_by_dest(mappings, min_value(mappings)).unwrap()
}

pub fn resolve_key(mappings: &Vec<Mapping>, key: u64) -> u64 {
    for map in mappings {
        if let Some(result) = map.map_source_to_destination(key) {
            return result;
        }
    }

    key
}

pub fn resolve_seed(mappings: &Vec<Vec<Mapping>>, seed: u64) -> u64 {
    let mut key = seed;

    for map in mappings {
        key = resolve_key(&map, key);
    }

    key
}

pub fn solution2(input: &str) -> Option<usize> {
    if let Some((seeds, maps)) = input.split_once("\n\n") {
        let ids = seeds
            .split_ascii_whitespace()
            .filter_map(|id| id.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let maps = maps
            .split("\n\n")
            .map(|m| {
                m.lines()
                    .skip(1)
                    .map(|l| {
                        l.split_ascii_whitespace()
                            .filter_map(|num| num.parse::<usize>().ok())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut arr = ids
            .chunks_exact(2)
            .map(|ele| ele[0]..(ele[0] + ele[1]))
            .collect::<Vec<_>>();

        for mut map in maps {
            map.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

            let mut idx = 0;
            loop {
                if let None = arr.get_mut(idx) {
                    break;
                }

                let current_range = arr[idx].clone();

                for m in map.iter() {
                    let destination = m[0];
                    let source = m[1];
                    let length = m[2];
                    let range = source..(source + length);

                    let current_start = current_range.start;
                    let current_end = current_range.end - 1;

                    let start_distance = current_start.saturating_sub(source);
                    let end_distance = current_end.saturating_sub(source);

                    if range.contains(&current_start) && range.contains(&current_end) {
                        arr[idx] = (destination + start_distance)..(destination + end_distance);

                        // println!("Range Contained -> {:?}", current_range);
                        // println!("Output Range -> {:?}", arr[idx]);

                        break;
                    } else if range.contains(&current_start) && !range.contains(&current_end) {
                        arr[idx] = (destination + start_distance)..(destination + length);
                        let next_range = (source + length)..current_end + 1;

                        // println!(
                        //     "Start Included, End Excluded -> Split Range: {:?} -> {:?}",
                        //     current_start..source + length,
                        //     next_range
                        // );
                        // println!("Output Range -> {:?}", arr[idx]);

                        arr.insert(idx + 1, next_range);
                        // solved.insert(idx + 1, false);
                        break;
                    } else if !range.contains(&current_start) && range.contains(&current_end) {
                        arr[idx] = (destination)..(destination + end_distance);
                        let next_range = (current_start)..(source);

                        // println!(
                        //     "Start Excluded, End Included -> Split Range: {:?} -> {:?}",
                        //     source..source + end_distance,
                        //     next_range
                        // );
                        // println!("Output Range -> {:?}", arr[idx]);

                        arr.insert(idx + 1, next_range);
                        // solved.insert(idx + 1, false);
                        break;
                    }
                    ()
                }
                idx += 1;
            }
        }

        let result = arr.iter().map(|r| r.start).min().unwrap();
        Some(result)
    } else {
        None
    }
}
