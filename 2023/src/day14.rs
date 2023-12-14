// 1 hour part 1, 1 hour part 2

use crate::utils::{Grid, GridRotation};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content)
        .rotate(GridRotation::Left);
    let grid = tilt(&grid)
        .rotate(GridRotation::Right);
    println!("PART 1: {}", score(&grid));
}


fn part2(content: &str) {
    let mut grid = Grid::from_string(content);
    let mut history = vec![grid.clone()];
    let mut loop_start = 0usize;
    loop {
        grid = cycle(&grid);
        for (i, prev) in history.iter().rev().enumerate() {
            if grid == *prev {
                loop_start = history.len() - i - 1;
                break;
            }
        }
        if loop_start > 0 {
            break;
        }
        history.push(grid.clone());
    }
    let warmup = history[..loop_start].len();
    let loop_size = history[loop_start..].len();
    let last = &history[warmup + (1_000_000_000_usize - warmup) % loop_size];
    println!("PART 2: {}", score(last));
}


fn cycle(grid: &Grid) -> Grid {
    let grid = tilt(&grid.rotate(GridRotation::Left));
    let grid = tilt(&grid.rotate(GridRotation::Right));
    let grid = tilt(&grid.rotate(GridRotation::Right));
    let grid = tilt(&grid.rotate(GridRotation::Right));
    grid.rotate(GridRotation::Flip)
}


fn tilt(grid: &Grid) -> Grid {
    // roll all 'O' to the left
    let mut cells = grid.cells.clone();
    for (r, row) in grid.cells.iter().enumerate() {
        let mut buffer: Vec<char> = vec![];
        for (i, c) in row.iter().enumerate() {
            if (*c == '#') & !buffer.is_empty() {
                buffer.sort();  // sorts '.' before 'O'
                for (offset, c) in buffer.iter().enumerate() {
                    cells[r][i - offset - 1] = *c;
                }
                buffer = vec![];
            } else if *c != '#' {
                buffer.push(*c);
            }
        }
        if !buffer.is_empty() {
            buffer.sort();  // sorts '.' before 'O'
            for (offset, c) in buffer.iter().enumerate() {
                cells[r][grid.ncols - offset - 1] = *c;
            }
        }
    }
    Grid::new(cells)
}


fn score(grid: &Grid) -> usize {
    let mut score = 0usize;
    for (offset, row) in grid.cells.iter().enumerate() {
        let mult = grid.nrows - offset;
        let nrocks: usize = row.iter().filter(|c| **c == 'O').count();
        score += mult * nrocks;
    }
    score
}