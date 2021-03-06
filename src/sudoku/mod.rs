use super::helpers::ExtendedIterator;
use crossterm::{terminal, ExecutableCommand};
use rand::prelude::*;
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::{thread, time};

#[derive(Debug)]
pub struct State {
    pub steps: u32,
    pub cells: [[u8; 9]; 9],

    pub rng: ThreadRng,
    pub allow_render: bool,
}

impl State {
    pub fn render_state(&self) {
        if self.allow_render {
            let mut stdout = stdout();
            stdout.write_all(render_sudoku(self.cells).as_bytes()).ok();
            let wait_time = time::Duration::from_millis(10);
            thread::sleep(wait_time);
            stdout
                .execute(terminal::Clear(terminal::ClearType::All))
                .ok();
        }
    }
}

pub fn render_sudoku(grid: [[u8; 9]; 9]) -> String {
    let border = "\n+-------+-------+-------+\n";

    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&val| {
                    if val == 0 {
                        " ".to_string()
                    } else {
                        val.to_string()
                    }
                })
                .grouped(3)
                .map(|mut group| group.mk_string("", " ", ""))
                .mk_string("| ", " | ", " |")
        })
        .grouped(3)
        .map(|mut group| group.mk_string("", "\n", ""))
        .mk_string(border, border, border)
}

/// Partially filled grids can be valid as well.
pub fn isvalid_sudoku(grid: [[u8; 9]; 9]) -> bool {
    let n = grid.len();

    for i in 0..n {
        let row: Vec<_> = grid[i].iter().filter(|&x| *x != 0).collect();
        let col: Vec<_> = grid
            .iter()
            .filter(|&line| line[i] != 0)
            .map(|line| line[i])
            .collect();
        let square: Vec<_> = (0..n)
            .map(|j| grid[i % 3 * 3 + j % 3][(i / 3) * 3 + j / 3])
            .filter(|&x| x != 0)
            .collect();

        if row.len() != row.iter().collect::<HashSet<_>>().len()
            || col.len() != col.iter().collect::<HashSet<_>>().len()
            || square.len() != square.iter().collect::<HashSet<_>>().len()
        {
            return false;
        }
    }

    true
}

pub fn solve(mut i: usize, mut j: usize, state: &mut State) -> bool {
    state.steps += 1;

    state.render_state();

    if i == 9 {
        j += 1;
        if j == 9 {
            return true;
        } else {
            i = 0
        }
    }

    // Already filled, move to next cell.
    if state.cells[i][j] != 0 {
        return solve(i + 1, j, state);
    }

    let mut values: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    values.shuffle(&mut state.rng);
    for &value in values.iter() {
        state.cells[i][j] = value;

        if isvalid_sudoku(state.cells) && solve(i + 1, j, state) {
            return true;
        }
    }

    state.cells[i][j] = 0;

    false
}

pub fn generate_sudoku(num_to_skip: usize) -> [[u8; 9]; 9] {
    // Randomly solve an empty grid first.
    let grid: [[u8; 9]; 9] = [[0; 9]; 9];

    let mut state = State {
        steps: 0,
        cells: grid,
        rng: thread_rng(),
        allow_render: false,
    };

    let _result = solve(0, 0, &mut state);

    // Shuffle array indices for positions to skip.
    let mut positions = Vec::<u8>::new();
    for index in 0..81 {
        positions.push(index);
    }
    let mut rng = thread_rng();
    positions.shuffle(&mut rng);

    // Set those randomly chosen positions to 0.
    for index in positions.iter().take(num_to_skip) {
        state.cells[(index / 9) as usize][(index % 9) as usize] = 0;
    }

    state.cells
}

pub fn run() {
    let mut rng = rand::thread_rng();
    let random_unfills = rng.gen_range(20, 40);

    let grid = generate_sudoku(random_unfills);
    let mut state = State {
        steps: 0,
        cells: grid,
        rng: rand::thread_rng(),
        allow_render: true,
    };

    solve(0, 0, &mut state);

    println!("INITIAL{}", render_sudoku(grid));
    println!("SOLVED{}", render_sudoku(state.cells));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sudoku() {
        // Create partial grid with 30 filled cells.
        let num_to_skip = 51;
        let grid = generate_sudoku(num_to_skip);
        assert!(isvalid_sudoku(grid));

        // Check if there are exactly 30 filled cells.
        let mut num_empty = 0;
        for row in grid.iter() {
            for &elem in row.iter() {
                if elem == 0 {
                    num_empty += 1;
                }
            }
        }

        assert_eq!(num_empty, num_to_skip);
    }

    #[test]
    fn test_partial_sudoku() {
        // Should be valid
        let grid: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [4, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        assert!(isvalid_sudoku(grid));
    }

    #[test]
    fn test_solve_empty_sudoku() {
        let grid: [[u8; 9]; 9] = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let mut state = State {
            steps: 0,
            cells: grid,
            rng: rand::thread_rng(),
            allow_render: false,
        };

        let result = solve(0, 0, &mut state);

        assert!(result);
    }

    #[test]
    fn test_solve_sudoku() {
        let grid: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let mut state = State {
            steps: 0,
            cells: grid,
            rng: rand::thread_rng(),
            allow_render: false,
        };

        let result = solve(0, 0, &mut state);

        assert!(result);

        let solution: [[u8; 9]; 9] = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        assert_eq!(solution, state.cells);
    }

    #[test]
    fn test_valid_sudoku() {
        let grid: [[u8; 9]; 9] = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        assert!(isvalid_sudoku(grid));
    }

    #[test]
    fn test_non_distinct() {
        let grid: [[u8; 9]; 9] = [
            [3, 1, 6, 5, 7, 8, 4, 9, 3], // non-distinct row (first row)
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        assert!(!isvalid_sudoku(grid));

        let grid: [[u8; 9]; 9] = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 2], // non-distinct col (last col)
        ];

        assert!(!isvalid_sudoku(grid));

        let grid: [[u8; 9]; 9] = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 8, 2, 6, 4, 3], // non-distinct square (middle square)
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        assert!(!isvalid_sudoku(grid));
    }

    #[test]
    fn test_invalid_sudoku() {
        let grid: [[u8; 9]; 9] = [
            [3, 1, 6, 5, 7, 8, 4, 9, 3],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 0, 1, 0, 0, 8, 0],
            [9, 7, 4, 8, 6, 3, 0, 0, 5],
            [8, 5, 1, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        assert!(!isvalid_sudoku(grid));
    }
}
