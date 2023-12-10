use std::{collections::HashMap, error::Error, u64};

use regex::Regex;

#[derive(Clone, Copy)]
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
}

pub fn parse_map_line(line: &str) -> Option<Mapping> {
    let re = Regex::new(r"(?P<dst_range_start>\d+)\s*(?P<src_range_start>\d+)\s*(?P<length>\d+)")
        .unwrap();

    println! {"{:?}", line};

    let capt = re.captures(line)?;
    let dst_range_start: u64 = capt.name("dst_range_start")?.as_str().parse().unwrap();
    let src_range_start: u64 = capt.name("src_range_start")?.as_str().parse().unwrap();
    let length: u64 = capt.name("length")?.as_str().parse().unwrap();

    Some(Mapping::new(src_range_start, dst_range_start, length))
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
