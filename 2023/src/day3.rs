// Yeesh, took 1.5 hours!

use std::collections::HashMap;
use crate::utils::Grid;


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &String) {
    let grid = Grid::new(content);
    
    let mut part_sum = 0;
    for (row, line) in grid.cells.iter().enumerate() {
        let mut buffer = 0;
        for (col, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                buffer = 10 * buffer + char.to_digit(10).unwrap();
            } else if buffer > 0 {
                let size = buffer.to_string().len();
                if is_near_symbol(&grid, row, col - size, col - 1) {
                    part_sum += buffer;
                };
                buffer = 0;
            } else {
                buffer = 0;
            }
        }
        if buffer > 0 {
            let size = buffer.to_string().len();
            if is_near_symbol(&grid, row, line.len() - size, line.len() - 1) {
                part_sum += buffer;
            }
      }
    }
    println!("PART 1: {}", part_sum);
}


fn is_near_symbol(grid: &Grid, row: usize, c1: usize, c2: usize) -> bool {
  let row_start = if row == 0 { 0 } else { row - 1 };
  for r in row_start..=row + 1 {
      let col_start = if c1 == 0 { 0 } else { c1 - 1 };
      for c in col_start..=c2 + 1 {
          if (r < grid.nrows) & (c < grid.ncols) {
              if is_symbol(&grid.cells[r][c]) {
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


fn part2(content: &String) {
    let grid = Grid::new(content);
    let mut gearmap: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    
    let mut buffer: u32 = 0;
    for (row, line) in grid.cells.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                buffer = 10 * buffer + char.to_digit(10).unwrap();
            }
            let next_col = col + 1;
            let finished = if next_col == grid.ncols { true } else { !grid.cells[row][next_col].is_digit(10) };
            if finished & (buffer > 0) {
                let size = buffer.to_string().len();
                let row_start = if row == 0 { 0 } else { row - 1 };
                for r in row_start..=row + 1 {
                    let c1 = col + 1 - size;
                    let col_start = if c1 == 0 { 0 } else { c1 - 1 };
                    for c in col_start..=col + 1 {
                        if (r < grid.nrows) & (c < grid.ncols) {
                            if grid.cells[r][c] == '*' {
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
    let gearratios: u32 = 
        gearmap.values()
        .filter(|vals| vals.len() == 2)
        .map(|vals| vals[0] * vals[1])
        .sum();

    println!("PART 2: {}", gearratios);
}
