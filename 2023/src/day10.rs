// 1.5 hours part 1

use std::collections::HashSet;

use crate::utils::{Grid, Point};


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::new(content);
    let start = find_start(&grid);
    let mut this = first(&grid, &start);
    let mut prev = start.clone();
    let mut length: usize = 1;
    loop {
        length += 1;
        let nxt = next(&grid, &this, &prev);
        prev = this;
        this = nxt;
        if this == start {
            break;
        }
    }
    println!("PART 1: {}", length / 2);
}


fn part2(_content: &str) {
    println!("PART 2: {}", -1);
}


fn find_start(grid: &Grid) -> Point {
    for r in 0usize..grid.nrows {
        for c in 0usize..grid.ncols {
            if grid.cells[r][c] == 'S' {
                return Point::new(r, c)
            }
        }
    }
    Point::new(0, 0)
}


fn first(grid: &Grid, start: &Point) -> Point {
    if HashSet::from(['|', 'F', '7']).contains(grid.get(&start.up())) {
        return start.up();
    } else if HashSet::from(['|', 'L', 'J']).contains(grid.get(&start.down())) {
        return start.down();
    } else if HashSet::from(['-', 'J', '7']).contains(grid.get(&start.right())) {
        return start.right();
    } else if HashSet::from(['-', 'F', 'L']).contains(grid.get(&start.left())) {
        return start.left();
    }
    dbg!(start);
    Point::new(0, 0)
}


fn next(grid: &Grid, curr: &Point, prev: &Point) -> Point {
    let cell = grid.get(curr);
    match *cell {
        '|' => return if curr.up() != *prev { curr.up() } else { curr.down() },
        '-' => return if curr.left() != *prev { curr.left() } else { curr.right() },
        'L' => return if curr.up() != *prev { curr.up() } else { curr.right() },
        'J' => return if curr.up() != *prev { curr.up() } else { curr.left() },
        '7' => return if curr.down() != *prev { curr.down() } else { curr.left() },
        'F' => return if curr.down() != *prev { curr.down() } else { curr.right() },
        _ => Point::new(0, 0)
    }
}