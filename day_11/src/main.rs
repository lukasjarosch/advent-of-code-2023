use std::time::Instant;

use itertools::Itertools;

#[allow(dead_code)]
const INPUT: &'static str = include_str!("../input2");

const EMPTY_SPACE: char = '.';
const GALAXY: char = '#';

fn main() {
    let time_start = Instant::now();
    println! {"=> Result part1: {:?}", solve(INPUT, 2)};
    println! {"=> Took: {:?}μs", time_start.elapsed().as_micros()};

    let time_start = Instant::now();
    println! {"=> Result part2: {:?}", solve(INPUT, 1000000)};
    println! {"=> Took: {:?}μs", time_start.elapsed().as_micros()};
}

fn solve(input: &str, expansion: u32) -> u64 {
    let galaxies = &expand_space(input, expansion);

    galaxies
        .into_iter()
        .enumerate()
        .fold(0, |mut acc, (i, galaxy)| {
            for next_galaxy in &galaxies[i + 1..] {
                acc += galaxy.distance(next_galaxy);
            }
            acc
        })
}

#[derive(Debug, Clone)]
struct Coordinate {
    row: u32,
    col: u32,
}

impl Coordinate {
    fn new(row: u32, col: u32) -> Coordinate {
        Coordinate { row, col }
    }

    fn distance(&self, other: &Coordinate) -> u64 {
        self.row.abs_diff(other.row) as u64 + self.col.abs_diff(other.col) as u64
    }
}

fn parse_space(input: &str) -> (Vec<Vec<char>>, Vec<Coordinate>) {
    let lines = input.lines().collect_vec();
    let mut space: Vec<Vec<char>> =
        vec![vec![EMPTY_SPACE; lines.first().unwrap().len()]; lines.len()];
    let mut galaxies: Vec<Coordinate> = vec![];

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, char)| {
            if char.eq(&GALAXY) {
                space[row][col] = GALAXY;
                galaxies.push(Coordinate::new(row as u32, col as u32));
            }
        })
    });

    (space, galaxies)
}

fn expand_space(input: &str, expansion_factor: u32) -> Vec<Coordinate> {
    let (space, mut galaxies) = parse_space(input);

    let empty_rows = space
        .clone()
        .into_iter()
        .enumerate()
        .filter_map(|(row, line)| {
            if line.iter().any(|c| c == &GALAXY) {
                None
            } else {
                Some(row as u32)
            }
        })
        .collect::<Vec<u32>>();

    let mut empty_cols: Vec<u32> = vec![];
    for col in 0..space[0].len() {
        let mut empty_column = true;
        for row in 0..space.len() {
            if space[row][col] == GALAXY {
                empty_column = false;
                break;
            }
        }
        if empty_column {
            empty_cols.push(col as u32);
        }
    }

    galaxies.iter_mut().for_each(|galaxy| {
        let (row, col) = (galaxy.row, galaxy.col);
        let rows_added =
            empty_rows.iter().filter(|r| r < &&row).count() as u32 * (expansion_factor - 1);
        let cols_added =
            empty_cols.iter().filter(|c| c < &&col).count() as u32 * (expansion_factor - 1);
        *galaxy = Coordinate::new(row + rows_added, col + cols_added);
    });

    galaxies
}
