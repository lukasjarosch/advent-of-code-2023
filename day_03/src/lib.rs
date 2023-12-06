use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub struct Position((usize, usize));

impl Position {
    pub fn row(&self) -> usize {
        self.0 .0
    }
    pub fn column(&self) -> usize {
        self.0 .1
    }
}

pub struct Matrix {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows().iter().enumerate() {
            write!(f, "{:?}\n", row)?;
        }
        write!(f, "")
    }
}

impl Matrix {
    pub fn new(input: &str) -> Matrix {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();

        let mut data: Vec<Vec<char>> = vec![vec!['\0'; cols]; rows];
        for (row, line) in input.lines().enumerate() {
            for (col, character) in line.chars().enumerate() {
                data[row][col] = character
            }
        }

        Matrix { data, rows, cols }
    }

    pub fn rows(&self) -> Vec<Vec<char>> {
        self.data.clone()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        Some(self.data.get(row)?.get(col)?.clone())
    }

    pub fn set(&mut self, row: usize, col: usize, data: char) {
        self.data[row][col] = data;
    }

    pub fn numbers(&self) -> Vec<(u16, Position)> {
        let mut numbers: Vec<(u16, Position)> = vec![];

        let mut num_buffer = String::new();

        for (i, row) in self.rows().iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                // 1. buffer empty, current char is numeric
                // 2. buffer empty, current char is NOT numeric
                if num_buffer.is_empty() {
                    if c.is_numeric() {
                        num_buffer += &c.to_string();
                    }
                    continue;
                }

                // 3. buffer NOT empty, current char is numeric
                // 4. buffer NOT empty, current char is NOT numeric
                // 5. buffer NOT empty, but we're at the end of this row (last column)
                if !num_buffer.is_empty() {
                    if c.is_numeric() {
                        num_buffer += &c.to_string();
                    }

                    if !c.is_numeric() {
                        let num_start = Position((i, j - num_buffer.len()));
                        numbers.push((num_buffer.parse().unwrap(), num_start));
                        num_buffer.clear();
                        continue;
                    }

                    if j == self.cols - 1 {
                        let num_start = Position((i, j + 1 - num_buffer.len()));
                        numbers.push((num_buffer.parse().unwrap(), num_start));
                        num_buffer.clear();
                        continue;
                    }
                }
            }
        }

        numbers
    }

    pub fn symbols(&self) -> Vec<(char, Position)> {
        let mut symbols: Vec<(char, Position)> = vec![];

        for (i, row) in self.rows().iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if !Matrix::is_symbol(c) {
                    continue;
                }

                symbols.push((*c, Position((i, j))));
            }
        }

        symbols
    }

    pub fn number_has_adjacent_symbol(&mut self, number: u16, pos: Position) -> bool {
        let positions_to_check = self.calculate_surrounding_positions(number, pos);

        for check_pos in positions_to_check {
            let c = self.get(check_pos.row(), check_pos.column()).unwrap();
            if Matrix::is_symbol(&c) {
                println! {"{number:>3} has adjacent symbol '{}' at ({},{})", c, check_pos.row(), check_pos.column()};
                return true;
            }
        }

        false
    }

    fn is_symbol(input: &char) -> bool {
        !(input.is_numeric() || input.to_string().eq("."))
    }

    fn calculate_surrounding_positions(&mut self, number: u16, pos: Position) -> Vec<Position> {
        // Given the number '123' all positions with '.' need to be checked
        // The Position points to '1'
        // . . . . .
        // . 1 2 3 .
        // . . . . .
        let mut positions_to_check: Vec<Position> = vec![];

        // calculate left border column
        let mut left_col = pos.column();
        if left_col > 0 {
            left_col = pos.column() - 1;
        }

        // calculate right border column
        let mut right_col = pos.column() + number.to_string().len() - 1;
        if right_col < self.cols - 1 {
            right_col += 1;
        }

        // calculate upper row border
        let mut upper_row = pos.row();
        if upper_row > 0 {
            upper_row -= 1;
        }

        // calculate lower row border
        let mut lower_row = pos.row();
        if lower_row < self.rows - 1 {
            lower_row += 1;
        }

        // calculate all positions
        for i in upper_row..=lower_row {
            for j in left_col..=right_col {
                positions_to_check.push(Position((i, j)))
            }
        }

        positions_to_check
    }
}
