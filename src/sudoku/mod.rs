use std::collections::HashSet;

pub fn isvalid_sudoku(grid: [[u32; 9]; 9]) -> bool {
    let n = grid.len();
    let mut isvalid = true;

    for i in 0..n {
        let row: HashSet<_> = grid[i].iter().filter(|&x| *x != 0).collect();
        let col: HashSet<_> = grid
            .iter()
            .filter(|&line| line[i] != 0)
            .map(|line| line[i])
            .collect();
        let square: HashSet<_> = (0..n)
            .map(|j| grid[i % 3 * 3 + j % 3][(i / 3) * 3 + j / 3])
            .filter(|&x| x != 0)
            .collect();

        isvalid = row.len() == n && col.len() == n && square.len() == n;
    }

    isvalid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isvalid_sudoku() {
        let grid: [[u32; 9]; 9] = [
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
}
