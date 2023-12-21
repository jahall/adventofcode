// 10 mins for part 1 - BLOODY HOURS for part 2!

use std::collections::{HashSet, HashMap};

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    let start = find_start(&grid);
    let possibilities = *brute(&grid, &start, 64).iter().last().unwrap();
    println!("PART 1: {}", possibilities);
}


fn part2(content: &str) {
    let grid = Grid::from_string(content);
    let evos = evolutions(&grid);
    
    // 1. Handle initial block
    let nsteps = 26_501_365;
    let mut count = get_state(&evos["."], nsteps);
    let width = grid.nrows - 1;

    // 2. Handle moving in perpendicular directions
    // It takes 66 steps to enter the first adjacent block to the middle one
    if nsteps >= width / 2 + 1 {
        for dir in ["n", "s", "e", "w"] {
            let mut n = nsteps - (width / 2 + 1);
            loop {
                count += get_state(&evos[dir], n);
                if n < width + 1 { break; }
                n = n - (width + 1);
            }
        }
    }

    // 3. Handle moving in diagonal directions
    // It takes 132 steps to enter the first diagonal block to the middle one
    if nsteps >= width + 2 {
        for dir in ["nw", "ne", "se", "sw"] {
            let mut n = nsteps - (width + 2);
            let mut i = 1;  // linearly increasing number of blocks along the edge
            loop {
                count += i * get_state(&evos[dir], n);
                if n < width + 1 { break; }
                n = n - (width + 1);
                i += 1;
            }
        }
    }
    println!("PART 2: {}", count);
}


/// Just use the middle
fn find_start(grid: &Grid) -> Point {
    Point::new((grid.nrows - 1) / 2, (grid.ncols - 1) / 2)
}


/// Brute force solver for a given start point
fn brute(grid: &Grid, start: &Point, nsteps: usize) -> Vec<usize> {
    let mut reachable = HashSet::new();
    let mut growth = vec![1];
    reachable.insert(*start);
    for _ in 1..=nsteps {
        let mut next = HashSet::new();
        for garden in reachable {
            next.extend(
                garden.direct_neighbors(&grid)
                .iter()
                .filter(|nbr| *grid.get(nbr) != '#')
            );
        }
        growth.push(next.len());
        if growth.len() > 2 && next.len() == growth[growth.len() - 3] {
            break;
        }
        reachable = next;
    }
    growth
}


/// Mapping from starting point to how things would evolve
fn evolutions(grid: &Grid) -> HashMap<String, Vec<usize>> {
    let e = grid.nrows - 1;
    let m = e / 2;
    HashMap::from_iter(
        [
            ("nw", 0, 0), ("n", 0, m), ("ne", 0, e),  // top
            ("w", m, 0), (".", m, m), ("e", m, e),  // middle
            ("sw", e, 0), ("s", e, m), ("se", e, e),  // bottom
        ]
        .map(|(dir, r, c)| (dir, Point::new(r, c)))
        .map(|(dir, p)| (String::from(dir), brute(grid, &p, 300)))
    )
}


/// Get the end-state, knowing that we end up flip-flopping
fn get_state(evolution: &[usize], step: usize) -> usize {
    let n = evolution.len();
    if step < n {
        evolution[step]
    } else {
        // e.g. step=n => rem=0 => evolution[-2]
        // or step=n+1 => rem=1 => evolution[-1]
        let rem = (step - n) % 2;
        evolution[n - 2 + rem]
    }
}


fn _show(grid: &Grid, points: &HashSet<Point>) {
    let mut grid = grid.clone();
    for r in 0..grid.nrows {
        for c in 0..grid.ncols {
            let p = Point::new(r, c);
            if points.contains(&p) {
                grid.cells[r][c] = 'o';
            }
        }
    }
    println!("\n{}\n", grid.to_string());
}