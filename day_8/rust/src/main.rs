use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let file = File::open("../../input.txt").expect("Readable file");
    let reader = BufReader::new(file);

    let grid: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|n| n.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut vis_grid = vec![vec![0; n_cols]; n_rows];

    let mut row_high = -1;
    let mut row_idx = 0;
    let mut col_high: Vec<i32> = vec![-1; n_cols];
    let mut col_idx: Vec<usize> = vec![0; n_cols];

    for (i, row) in grid.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            // Check column (top to bottom)
            if *num > col_high[j] {
                vis_grid[i][j] = 1;
                col_high[j] = *num;
                col_idx[j] = i;
            }

            // Check row (left to right)
            if *num > row_high {
                vis_grid[i][j] = 1;
                row_idx = j;
                row_high = *num;
            }
        }

        // Check row (right to left)
        row_high = -1;
        for (j, num) in row[row_idx + 1..].into_iter().rev().enumerate() {
            if *num > row_high {
                row_high = *num;
                vis_grid[i][n_cols - 1 - j] = 1;
            }
        }
        row_high = -1;
    }

    // Check column (bottom to top)
    let min_col = col_idx.iter().min().unwrap();
    col_high.fill(-1);
    for (i, row) in grid[*min_col + 1..].iter().rev().enumerate() {
        for (j, num) in row.iter().enumerate() {
            if *num > col_high[j] {
                vis_grid[n_rows - 1 - i][j] = 1;
                col_high[j] = *num;
            }
        }
    }
    let result: i32 = vis_grid.iter().map(|v| v.iter().sum::<i32>()).sum();

    println!("{:?}", result);
}
