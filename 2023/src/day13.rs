// ~1 hour all in...bloody fiddly

use crate::utils::Grid;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grids = parse_content(content);
    let solution: usize = grids.iter().map(|g| solve(g, &0)).sum();
    println!("PART 1: {}", solution);
}


fn part2(content: &str) {
    let mut grids = parse_content(content);
    let solution: usize = grids.iter_mut().map(solve_smudge).sum();
    println!("PART 2: {}", solution);
}


fn solve_smudge(grid: &mut Grid) -> usize {
    let original = solve(grid, &0);
    for r in 0..grid.nrows {
        for c in 0..grid.ncols {
            // flip
            grid.cells[r][c] = if grid.cells[r][c] == '.' { '#' } else { '.' };
            // check
            let solution = solve(grid, &original);
            if (solution > 0) & (solution != original) {
                return solution;
            }
            // unflip
            grid.cells[r][c] = if grid.cells[r][c] == '.' { '#' } else { '.' };
        }
    }
    0
}


fn solve(grid: &Grid, original: &usize) -> usize {
    // check cols
    for line in 1..grid.ncols {        
        let mut fail = false;
        for offset in 0..=line {
            if (offset + 1 > line) | (offset + line >= grid.ncols) {
                break;
            }
            if !cols_equal(grid, line - 1 - offset, line + offset) {
                fail = true;
                break;
            }
        }
        if !fail & (line != *original) {
            return line;
        }
    }
    // check rows
    for line in 1..grid.nrows {        
        let mut fail = false;
        for offset in 0..=line {
            if (offset + 1 > line) | (offset + line >= grid.nrows) {
                break;
            }
            if !rows_equal(grid, line - 1 - offset, line + offset) {
                fail = true;
                break;
            }
        }
        if !fail & (100 * line != *original) {
            return 100 * line;
        }
    }
    0
}


fn cols_equal(grid: &Grid, c1: usize, c2: usize) -> bool {
    grid.cells.iter().all(|r| r[c1] == r[c2])
}


fn rows_equal(grid: &Grid, r1: usize, r2: usize) -> bool {
    grid.cells[r1] == grid.cells[r2]
}


fn parse_content(content: &str) -> Vec<Grid> {
    let mut grids: Vec<Grid> = vec![];
    let mut buffer: Vec<&str> = vec![];
    for row in content.split("\n") {
        if row == "" {
            grids.push(Grid::from_string(&buffer.join("\n")));
            buffer = vec![];
        } else {
            buffer.push(row);
        }
    }
    grids.push(Grid::from_string(&buffer.join("\n")));
    grids
}