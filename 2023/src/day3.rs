// Yeesh, took 1.5 hours!

use std::collections::HashMap;


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &String) {
    let grid: Vec<Vec<char>> = content.split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();
    
    let mut part_sum = 0;
    for (row, line) in grid.iter().enumerate() {
        let mut buffer = 0;
        for (col, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                buffer = 10 * buffer + char.to_digit(10).unwrap();
            } else if buffer > 0 {
                let size = buffer.to_string().len();
                if check(&grid, row, col - size, col - 1) {
                    part_sum += buffer;
                };
                buffer = 0;
            } else {
                buffer = 0;
            }
        }
        if buffer > 0 {
            let size = buffer.to_string().len();
            if check(&grid, row, line.len() - size, line.len() - 1) {
                part_sum += buffer;
            }
      }
    }
    println!("PART 1: {}", part_sum);
}


fn part2(content: &String) {
    let grid: Vec<Vec<char>> = content.split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut gearmap: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    
    let mut buffer: u32 = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                buffer = 10 * buffer + char.to_digit(10).unwrap();
            }
            let next_col = col + 1;
            let finished = if next_col == ncols { true } else { !grid[row][next_col].is_digit(10) };
            if finished & (buffer > 0) {
                let size = buffer.to_string().len();
                let row_start = if row == 0 { 0 } else { row - 1 };
                for r in row_start..=row + 1 {
                    let c1 = col + 1 - size;
                    let col_start = if c1 == 0 { 0 } else { c1 - 1 };
                    for c in col_start..=col + 1 {
                        if (r < nrows) & (c < ncols) {
                            if grid[r][c] == '*' {
                                let key = (r, c);
                                gearmap
                                    .entry(key)
                                    .and_modify(|v| v.push(buffer))
                                    .or_insert(vec![buffer]);
                            }
                        }
                    }
                }
                buffer = 0;
            }
        }
    }
    let mut gearratios = 0;
    for item in gearmap.values() {
        if item.len() == 2 {
            gearratios += item[0] * item[1];
        }
    }
    println!("PART 2: {}", gearratios);
}


fn check(grid: &Vec<Vec<char>>, row: usize, c1: usize, c2: usize) -> bool {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let row_start = if row == 0 { 0 } else { row - 1 };
    for r in row_start..=row + 1 {
        let col_start = if c1 == 0 { 0 } else { c1 - 1 };
        for c in col_start..=c2 + 1 {
            if (r < nrows) & (c < ncols) {
                if is_symbol(&grid[r][c]) {
                    return true;
                }
            }
        }
    }
    false
}


fn is_symbol(char: &char) -> bool {
    let syms = ['$', '#', '&', '%', '@', '+', '-', '*', '/', '='];
    for sym in syms {
        if sym == *char {
            return true;
        }
    }
    false
}