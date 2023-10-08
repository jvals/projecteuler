use itertools::Itertools;
use rayon::prelude::*;
use std::error::Error;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
struct SudokuCell {
    value: u8,
    possible_values: Vec<u8>,
}

impl SudokuCell {
    fn new(value: u8) -> Self {
        SudokuCell {
            value,
            possible_values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
                .into_iter()
                .filter(|&x| x != value)
                .collect(),
        }
    }

    fn prune(&mut self, value: u8) {
        self.possible_values.retain(|&x| x != value);
        if self.possible_values.len() == 1 {
            self.value = self.possible_values[0];
        }
    }
}

impl Default for SudokuCell {
    fn default() -> Self {
        SudokuCell::new(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SudokuBoard {
    name: String,
    cells: Vec<SudokuCell>,
}

impl SudokuBoard {
    fn new(name: &str) -> Self {
        SudokuBoard {
            name: name.to_string(),
            cells: vec![SudokuCell::default(); 81],
        }
    }

    /*
    parse inputs like the following grid
    003020600
    900305001
    001806400
    008102900
    700000008
    006708200
    002609500
    800203009
    005010300
    */
    fn parse(name: &str, input: &str) -> Self {
        let mut board = SudokuBoard::new(name);
        let mut position = 0;
        for line in input.lines() {
            for c in line.chars() {
                if let Some(digit) = c.to_digit(10) {
                    board.cells[position].value = digit as u8;
                    position += 1;
                }
            }
        }
        board
    }

    #[allow(dead_code)]
    fn get_board_as_string(&self) -> String {
        let mut board_string = String::new();
        for row in self.cells.chunks(9) {
            for cell in row {
                board_string.push_str(&cell.value.to_string());
            }
            board_string.push('\n');
        }
        board_string
    }

    #[allow(dead_code)]
    fn print_board(&self) {
        println!("Board:");
        println!("{}", self.get_board_as_string());
    }

    #[allow(dead_code)]
    fn pretty_print_board(&self) {
        println!("{}:", self.name);
        for (i, row) in self.cells.chunks(9).enumerate() {
            if i % 3 == 0 {
                println!("-------------------------");
            }
            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 {
                    print!("| ");
                }
                if cell.value == 0 {
                    print!(". ");
                } else {
                    print!("{} ", cell.value);
                }
            }
            println!("|");
        }
        println!("-------------------------");
    }

    // prune possible values for each cell
    fn prune(&mut self) {
        // prune rows
        for row in self.cells.chunks_mut(9) {
            for cell_idx in 0..row.len() {
                for other_cell_idx in 0..row.len() {
                    if cell_idx != other_cell_idx && row[cell_idx].value != 0 {
                        let cell_value = row[cell_idx].value;
                        let other_cell = &mut row[other_cell_idx];
                        other_cell.prune(cell_value);
                    }
                }
            }
        }
        // prune columns
        for col in 0..9 {
            for row_idx in 0..9 {
                for other_row_idx in 0..9 {
                    if row_idx != other_row_idx && self.cells[row_idx * 9 + col].value != 0 {
                        let cell_value = self.cells[row_idx * 9 + col].value;
                        let other_cell = &mut self.cells[other_row_idx * 9 + col];
                        other_cell.prune(cell_value);
                    }
                }
            }
        }
        // prune boxes
        for box_idx in 0..9 {
            let box_row = box_idx / 3;
            let box_col = box_idx % 3;
            for cell_idx in 0..9 {
                let cell_row = cell_idx / 3;
                let cell_col = cell_idx % 3;
                let cell_value =
                    self.cells[(box_row * 3 + cell_row) * 9 + box_col * 3 + cell_col].value;
                for other_cell_idx in 0..9 {
                    if cell_idx != other_cell_idx && cell_value != 0 {
                        let other_cell_row = other_cell_idx / 3;
                        let other_cell_col = other_cell_idx % 3;
                        let other_cell = &mut self.cells
                            [(box_row * 3 + other_cell_row) * 9 + box_col * 3 + other_cell_col];
                        other_cell.prune(cell_value);
                    }
                }
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.is_full() && self.is_valid()
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|cell| cell.value != 0)
    }

    fn is_valid(&self) -> bool {
        // check that each row contains all digits
        for row in self.cells.chunks(9) {
            let mut found_digits = vec![false; 9];
            for cell in row {
                if cell.value != 0 {
                    found_digits[(cell.value - 1) as usize] = true;
                }
            }
            if found_digits.iter().any(|&x| !x) {
                return false;
            }
        }

        // check that each column contains all digits
        for col in 0..9 {
            let mut found_digits = vec![false; 9];
            for row in 0..9 {
                let cell = &self.cells[row * 9 + col];
                if cell.value != 0 {
                    found_digits[(cell.value - 1) as usize] = true;
                }
            }
            if found_digits.iter().any(|&x| !x) {
                return false;
            }
        }

        // check that each box contains all digits
        for box_idx in 0..9 {
            let mut found_digits = vec![false; 9];
            let box_row = box_idx / 3;
            let box_col = box_idx % 3;
            for cell_idx in 0..9 {
                let cell_row = cell_idx / 3;
                let cell_col = cell_idx % 3;
                let cell = &self.cells[(box_row * 3 + cell_row) * 9 + box_col * 3 + cell_col];
                if cell.value != 0 {
                    found_digits[(cell.value - 1) as usize] = true;
                }
            }
            if found_digits.iter().any(|&x| !x) {
                return false;
            }
        }

        true
    }

    fn repeated_prune(&mut self) {
        // repeat pruning until the board is solved or the board is not changed
        let mut last_board = self.clone();
        loop {
            self.prune();
            if self.is_solved() {
                break;
            }
            if self == &last_board {
                break;
            }
            last_board = self.clone();
        }
    }

    fn get_cell_with_least_possible_values(&self) -> (usize, &SudokuCell) {
        // get the cell with the least number of possible values that is not solved
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.value == 0)
            .min_by_key(|(_, cell)| cell.possible_values.len())
            .unwrap()
    }
}

fn solve(board: SudokuBoard) -> Result<SudokuBoard, Box<dyn Error>> {
    // Apply repeated pruning
    let pruned_board = {
        let mut board = board;
        board.repeated_prune();
        board
    };

    if pruned_board.is_solved() || pruned_board.is_full() {
        return Ok(pruned_board);
    }

    // Find the cell with the least number of possible values
    let (cell_idx, cell) = pruned_board.get_cell_with_least_possible_values();

    // Guess a value and recursively apply the solve function
    for value in cell.possible_values.iter() {
        let mut next_board = pruned_board.clone();
        next_board.cells[cell_idx].value = *value;
        if let Ok(solved_board) = solve(next_board) {
            if solved_board.is_solved() {
                return Ok(solved_board);
            }
        }
    }

    Err("Cannot solve the board".into())
}

fn parse_p096_sudoku() -> Vec<SudokuBoard> {
    // parse p096_sudoku.txt into a vector of boards
    let all_boards_raw = include_str!("../p096_sudoku.txt");
    // read 10 lines at a time and parse them into a board
    all_boards_raw
        .lines()
        .chunks(10)
        .into_iter()
        .map(|mut board_raw| {
            // Each board starts with a line that starts with "Grid xx". Use this as the board name
            let board_name = board_raw.next().unwrap();
            // Then read the next 9 lines into a single string
            let board_raw = board_raw.take(9).join("");

            SudokuBoard::parse(board_name, &board_raw)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let boards = parse_p096_sudoku();
    let sum: u32 = boards
        .into_par_iter()
        .map(|board| {
            let solved_board = solve(board).unwrap();
            let first_3_digits = solved_board.cells[0..3]
                .iter()
                .fold(0, |acc, cell| acc * 10 + cell.value as u32);
            first_3_digits
        })
        .sum();
    let elapsed = start_time.elapsed();

    println!("Execution time: {}ms", elapsed.as_millis());
    println!("Sum of first 3 digits of the 50 solved boards: {}", sum);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "003020600
                   900305001
                   001806400
                   008102900
                   700000008
                   006708200
                   002609500
                   800203009
                   005010300";

    #[test]
    fn test_board() {
        let mut board = SudokuBoard::parse("name", INPUT);
        board.print_board();
        board.pretty_print_board();
        board.repeated_prune();
        board.pretty_print_board();
    }

    #[test]
    fn test_prune() {
        let mut board = SudokuBoard::parse("name", INPUT);
        board.prune();
        let expected = SudokuBoard::parse(
            "name",
            "003020600
                   900305001
                   001806400
                   008102900
                   700004108
                   006708200
                   002689500
                   800253009
                   005417300",
        );
        assert_eq!(board.get_board_as_string(), expected.get_board_as_string());
    }

    #[test]
    fn test_repeated_prune() {
        let mut board = SudokuBoard::parse("name", INPUT);
        board.repeated_prune();
        let expected = SudokuBoard::parse(
            "name",
            "483921657
                   967345821
                   251876493
                   548132976
                   729564138
                   136798245
                   372689514
                   814253769
                   695417382",
        );
        assert_eq!(board.get_board_as_string(), expected.get_board_as_string());
    }

    #[test]
    fn test_box_iteration() {
        for box_idx in 0..9 {
            let box_row = box_idx / 3;
            let box_col = box_idx % 3;
            for cell_idx in 0..9 {
                let cell_row = cell_idx / 3;
                let cell_col = cell_idx % 3;
                println!("box_idx: {}, box_row: {}, box_col: {}, cell_idx: {}, cell_row: {}, cell_col: {}", box_idx, box_row, box_col, cell_idx, cell_row, cell_col);
            }
        }
    }

    #[test]
    fn test_board_eq() {
        // test that two boards are equal
        let board1 = SudokuBoard::parse("name", INPUT);
        let board2 = SudokuBoard::parse("name", INPUT);
        assert_eq!(board1, board2);
    }

    #[test]
    fn test_parse_p096_sudoku() {
        let boards = parse_p096_sudoku();
        assert_eq!(boards.len(), 50);
        let first_board = boards.first().unwrap();
        first_board.pretty_print_board();
        assert_eq!(
            first_board.get_board_as_string(),
            SudokuBoard::parse("name", INPUT).get_board_as_string()
        );
    }

    #[test]
    fn test_is_solved() {
        let board = SudokuBoard::parse("name", INPUT);
        assert!(!board.is_solved());
        let solved_board = SudokuBoard::parse(
            "name",
            "483921657
                   967345821
                   251876493
                   548132976
                   729564138
                   136798245
                   372689514
                   814253769
                   695417382",
        );
        assert!(solved_board.is_solved());
    }
}
