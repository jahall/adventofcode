use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

use itertools::Itertools;

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    println!("PART 1: {}", find_path(&grid));
}


fn part2(_content: &str) {
    println!("PART 2: {}", -1);
}


/// Use Djikastra to find shortest path
fn find_path(grid: &Grid) -> usize {

    // initialize the heap
    let mut heap = BinaryHeap::new();
    heap.push(State::start());

    // initialize the dists
    let mut dists: HashMap<Point, usize> =
        (0..grid.nrows)
        .cartesian_product(0..grid.ncols)
        .map(|x| (Point::new(x.0, x.1), usize::MAX))
        .collect();
    dists.insert(State::start().loc, 0);

    // define the target
    let target = Point::new(grid.nrows - 1, grid.ncols - 1);

    while let Some(state) = heap.pop() {
        // we've arrived at the target!
        if state.loc == target {
            show(grid, &state);
            return state.cost;
        }

        // handle new history
        let mut prev = vec![state.loc];
        if !state.prev.is_empty() { prev.extend(state.prev.iter()); }

        // add neighbors to the heap
        for neighbor in state.neighbors(grid) {
            let next = State::new(
                state.cost + grid.getnum(&neighbor),
                neighbor,
                prev.clone(),
            );
            if next.cost < dists[&next.loc] {
                dists.insert(next.loc, next.cost);
                heap.push(next);
            }
        }
    }
    dists[&target]
}


fn show(grid: &Grid, state: &State) {
    let mut grid = grid.clone();
    for p in state.prev.iter() {
        grid.cells[p.r][p.c] = '.';
    }
    println!("{}", grid.to_string());
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    loc: Point,
    prev: Vec<Point>,
}

impl State {
    fn start() -> State {
        State{ cost: 0, loc: Point::new(0, 0), prev: vec![] }
    }

    fn new(cost: usize, loc: Point, prev: Vec<Point>) -> State {
        State{ cost, loc, prev }
    }

    /// Valid neighbors - ensuring we don't go off-grid or go more than 3 steps in one direction
    fn neighbors(&self, grid: &Grid) -> Vec<Point> {
        let (allow_vertical, allow_horizontal) =
            if self.prev.len() < 3 {
                (true, true)
            } else {
                (
                    // not been in this column too long
                    self.prev[..3].iter().any(|p| p.c != self.loc.c),
                    // not been in this row too long
                    self.prev[..3].iter().any(|p| p.r != self.loc.r),
                )
            };
        
        let mut neighbors: Vec<Point> = vec![];
        if allow_vertical {
            if self.loc.r > 0 { neighbors.push(self.loc.up()); }
            if self.loc.r < grid.nrows - 1 { neighbors.push(self.loc.down()); }
        };
        if allow_horizontal {
            if self.loc.c > 0 { neighbors.push(self.loc.left()); }
            if self.loc.c < grid.ncols - 1 { neighbors.push(self.loc.right()); }
        };
        neighbors
    }
}

/// Ordering required for heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the ordering on costs so it becomes a min-heap
        other.cost.cmp(&self.cost)
            .then_with(|| self.loc.r.cmp(&other.loc.r))
            .then_with(|| self.loc.c.cmp(&other.loc.c))
    }
}

// PartialOrd also required for heap
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}