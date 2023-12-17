use std::{char, collections::HashMap, fmt::Debug, ops::Deref, ops::DerefMut};

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    PipeVerticalNorthSouth,
    PipeHorizontalEastWest,
    PipeBendNorthEast,
    PipeBendNorthWest,
    PipeBendSouthWest,
    PipeBendSouthEast,
    Ground,
    StartPosition,
}

impl Tile {
    pub fn value(&self) -> char {
        match *self {
            Tile::PipeVerticalNorthSouth => '|',
            Tile::PipeHorizontalEastWest => '-',
            Tile::PipeBendNorthEast => 'L',
            Tile::PipeBendNorthWest => 'J',
            Tile::PipeBendSouthWest => '7',
            Tile::PipeBendSouthEast => 'F',
            Tile::Ground => '.',
            Tile::StartPosition => 'S',
        }
    }
    pub fn from_char(c: char) -> Tile {
        match c {
            '|' => Tile::PipeVerticalNorthSouth,
            '-' => Tile::PipeHorizontalEastWest,
            'L' => Tile::PipeBendNorthEast,
            'J' => Tile::PipeBendNorthWest,
            '7' => Tile::PipeBendSouthWest,
            'F' => Tile::PipeBendSouthEast,
            '.' => Tile::Ground,
            'S' => Tile::StartPosition,
            _ => panic!("invalid char"),
        }
    }

    fn valid_connections(&self, direction: Direction) -> Option<Vec<Tile>> {
        let lookup: Vec<(Tile, Direction, Vec<Tile>)> = vec![
            (
                Tile::PipeVerticalNorthSouth,
                Direction::North,
                vec![
                    Tile::PipeVerticalNorthSouth,
                    Tile::PipeBendSouthEast,
                    Tile::PipeBendSouthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeVerticalNorthSouth,
                Direction::South,
                vec![
                    Tile::PipeVerticalNorthSouth,
                    Tile::PipeBendNorthEast,
                    Tile::PipeBendNorthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeHorizontalEastWest,
                Direction::East,
                vec![
                    Tile::PipeHorizontalEastWest,
                    Tile::PipeBendNorthWest,
                    Tile::PipeBendSouthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeHorizontalEastWest,
                Direction::West,
                vec![
                    Tile::PipeHorizontalEastWest,
                    Tile::PipeBendNorthEast,
                    Tile::PipeBendSouthEast,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendNorthEast,
                Direction::North,
                vec![
                    Tile::PipeVerticalNorthSouth,
                    Tile::PipeBendSouthEast,
                    Tile::PipeBendSouthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendNorthEast,
                Direction::East,
                vec![
                    Tile::PipeHorizontalEastWest,
                    Tile::PipeBendNorthWest,
                    Tile::PipeBendSouthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendNorthWest,
                Direction::North,
                vec![
                    Tile::PipeVerticalNorthSouth,
                    Tile::PipeBendSouthWest,
                    Tile::PipeBendSouthEast,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendNorthWest,
                Direction::West,
                vec![
                    Tile::PipeHorizontalEastWest,
                    Tile::PipeBendNorthEast,
                    Tile::PipeBendSouthEast,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendSouthWest,
                Direction::South,
                vec![
                    Tile::PipeVerticalNorthSouth,
                    Tile::PipeBendNorthEast,
                    Tile::PipeBendNorthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendSouthWest,
                Direction::West,
                vec![
                    Tile::PipeHorizontalEastWest,
                    Tile::PipeBendNorthEast,
                    Tile::PipeBendSouthEast,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendSouthEast,
                Direction::East,
                vec![
                    Tile::PipeHorizontalEastWest,
                    Tile::PipeBendSouthWest,
                    Tile::PipeBendNorthWest,
                    Tile::StartPosition,
                ],
            ),
            (
                Tile::PipeBendSouthEast,
                Direction::South,
                vec![
                    Tile::PipeVerticalNorthSouth,
                    Tile::PipeBendNorthWest,
                    Tile::PipeBendNorthEast,
                    Tile::StartPosition,
                ],
            ),
        ];

        if self == &Tile::StartPosition {
            return Some(vec![
                Tile::PipeVerticalNorthSouth,
                Tile::PipeHorizontalEastWest,
                Tile::PipeBendNorthEast,
                Tile::PipeBendNorthWest,
                Tile::PipeBendSouthWest,
                Tile::PipeBendSouthEast,
                Tile::Ground,
                Tile::StartPosition,
            ]);
        }

        if let Some(found) = lookup.iter().find(|x| &x.0 == self && x.1 == direction) {
            return Some(found.2.clone());
        }

        None
    }

    pub fn can_connect(a: Tile, b: Tile, direction: Direction) -> bool {
        if b == Tile::Ground {
            return false;
        }

        if let Some(valid_connections) = Tile::valid_connections(&a, direction) {
            return match valid_connections.iter().find(|x| **x == b) {
                None => false,
                Some(_) => true,
            };
        }
        return false;
    }
}

pub struct Field(pub Vec<Vec<Tile>>);

impl Field {
    pub fn tile_index(&self, row: usize, col: usize) -> usize {
        // row * width + col
        row * self.0.iter().next().unwrap().len() + col
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[Field {} x {}]\n",
            self.0.len(),
            self.0.iter().count()
        ))
        .unwrap();
        for line in &self.0 {
            for c in line {
                f.write_str(&c.value().to_string()).unwrap();
            }
            f.write_str("\n").unwrap();
        }

        Ok(())
    }
}

impl Deref for Field {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Field {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
