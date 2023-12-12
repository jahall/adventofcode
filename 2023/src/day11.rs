// 30 mins part 1, 5 mins part 2 :)

use std::collections::HashSet;

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    println!("PART 1: {}", solve(&content, 2));
    println!("PART 2: {}", solve(&content, 1000000));
}


fn solve(content: &str, expansion: usize) -> i64 {
    let grid = Grid::new(content);
    let planets = fetch_planets(&grid, expansion);
    let mut dists = 0i64;
    for i in 0..planets.len() {
        for j in i + 1..planets.len() {
            dists += calc_distance(&planets[i], &planets[j]);
        }
    }
    dists
}


fn fetch_planets(grid: &Grid, expansion: usize) -> Vec<Point> {
    // 1. find non empty rows and cols
    let mut planets: Vec<Point> = vec![];
    let mut non_empty_rows: HashSet<usize> = HashSet::new();
    let mut non_empty_cols: HashSet<usize> = HashSet::new();
    for (r, row) in grid.cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == '#' {
                planets.push(Point::new(r, c));
                non_empty_rows.insert(r);
                non_empty_cols.insert(c);
            }
        }
    }
    // 2. get new rows
    let mut newr = vec![0usize; grid.nrows];
    let mut offset = 0usize;
    for r in 0..grid.nrows {
        if non_empty_rows.contains(&r) {
            newr[r] = r + offset;
        } else {
            offset += expansion - 1;
        }
    }
    // 3. get new cols
    let mut newc= vec![0usize; grid.ncols];
    offset = 0;
    for c in 0..grid.ncols {
        if non_empty_cols.contains(&c) {
            newc[c] = c + offset;
        } else {
            offset += expansion - 1;
        }
    }
    // 4. re-map planet coords
    for i in 0..planets.len() {
        planets[i] = Point::new(
            newr[planets[i].r],
            newc[planets[i].c],
        );
    }
    planets
}


fn calc_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.r as i64 - p2.r as i64).abs() + (p1.c as i64 - p2.c as i64).abs()
}