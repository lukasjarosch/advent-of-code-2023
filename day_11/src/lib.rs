use std::{char, fmt::Debug, ops::Deref, ops::DerefMut, usize, vec};

pub struct Space {
    pub data: Vec<Vec<char>>,
    pub galaxies: Vec<Galaxy>,
}

const GALAXY: char = '#';
const EMPTY_SPACE: char = '.';

impl Space {
    pub fn new(input: String) -> Space {
        let len = input.lines().into_iter().next().unwrap().len();
        let height = input.lines().count();

        let mut data: Vec<Vec<char>> = vec![vec!['.'; len]; height];
        for (i, line) in input.lines().enumerate() {
            data[i] = line.chars().collect();
        }

        let mut space = Space {
            data,
            galaxies: vec![],
        };

        space.reset_galaxies();
        space
    }

    fn galaxy_positions(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = vec![];
        for (i, row) in self.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col.eq(&GALAXY) {
                    positions.push((i, j))
                }
            }
        }

        positions
    }

    fn reset_galaxies(&mut self) {
        self.galaxies.clear();
        for (i, pos) in self.galaxy_positions().iter().enumerate() {
            let galaxy = Galaxy::new(i.to_string(), *pos);
            self.galaxies.push(galaxy);
        }
    }

    pub fn expand(&mut self) {
        let galaxy_rows: Vec<usize> = self
            .galaxy_positions()
            .into_iter()
            .map(|(row, _)| row)
            .collect();
        let galaxy_columns: Vec<usize> = self
            .galaxy_positions()
            .into_iter()
            .map(|(_, column)| column)
            .collect();
        let empty_rows: Vec<usize> = (0..self.rows())
            .into_iter()
            .filter(|x| !galaxy_rows.contains(&x))
            .collect();
        let empty_columns: Vec<usize> = (0..self.columns())
            .into_iter()
            .filter(|x| !galaxy_columns.contains(&x))
            .collect();

        for (idx, empty_row) in empty_rows.iter().enumerate() {
            self.data
                .insert(empty_row + idx, vec![EMPTY_SPACE; self.columns()]);
        }

        for (idx, empty_column) in empty_columns.iter().enumerate() {
            for row in self.data.iter_mut() {
                row.insert(empty_column + idx, EMPTY_SPACE)
            }
        }

        self.reset_galaxies();
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn columns(&self) -> usize {
        self.data.iter().next().unwrap().len()
    }

    pub fn galaxy_at(&self, pos: (usize, usize)) -> Option<&Galaxy> {
        self.galaxies
            .iter()
            .find(|x| x.position.0 == pos.0 && x.position.1 == pos.1)
    }

    pub fn galaxy_pairs(&self) -> Vec<(&Galaxy, &Galaxy)> {
        let mut pairs: Vec<(&Galaxy, &Galaxy)> = vec![];
        for (i, g) in self.galaxies.iter().enumerate() {
            for g2 in self.galaxies.iter().skip(i + 1) {
                pairs.push((g, g2));
            }
        }
        pairs
    }

    // distance calculates the euclidean distance between two points
    // abs(x1 - x2) + abs(y1 - y2)
    pub fn distance(&self, pos1: (usize, usize), pos2: (usize, usize)) -> usize {
        let x_diff = pos1.0.abs_diff(pos2.0);
        let y_diff = pos1.1.abs_diff(pos2.1);
        println! {"{:?} -> {:?} = {:?}", pos1, pos2, x_diff + y_diff};

        x_diff + y_diff
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n").unwrap();
        for (i, line) in (&self.data).iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if let Some(galaxy) = self.galaxy_at((i, j)) {
                    f.write_str(&galaxy.name).unwrap();
                    continue;
                }

                f.write_str(&c.to_string()).unwrap();
            }
            f.write_str("\n").unwrap();
        }

        Ok(())
    }
}

impl Deref for Space {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Space {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Galaxy {
    pub name: String,
    pub position: (usize, usize),
}

impl Galaxy {
    pub fn new(name: String, position: (usize, usize)) -> Galaxy {
        Galaxy { name, position }
    }
}
