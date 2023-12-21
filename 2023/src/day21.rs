// 10 mins for part 1

use std::collections::HashSet;

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    let start = find_start(&grid);
    let mut reachable = HashSet::new();
    reachable.insert(start);
    for _ in 1..=64 {
        let mut next = HashSet::new();
        for garden in reachable {
            next.extend(
                garden.direct_neighbors(&grid)
                .iter()
                .filter(|nbr| *grid.get(nbr) != '#')
            );
        }
        reachable = next;
    }
    println!("PART 1: {}", reachable.len());
}


fn part2(_content: &str) {
    println!("PART 2: {}", -1);
}


fn find_start(grid: &Grid) -> Point {
    for (r, row) in grid.cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                return Point::new(r, c);
            }
        }
    }
    panic!("Couldn't find the start!")
}