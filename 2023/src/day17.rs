// SO MANY HOURS! figuring out the state and key representations for djikastra!
use std::{collections::{BinaryHeap, HashMap}, cmp::Ordering};

use itertools::Itertools;

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    println!("PART 1: {}", find_path(&grid, false, false));
}


fn part2(content: &str) {
    let grid = Grid::from_string(content);
    println!("PART 2: {}", find_path(&grid, true, false));
}


/// Use Djikastra to find shortest path
fn find_path(grid: &Grid, is_ultra: bool, show_path: bool) -> usize {

    // initialize the heap
    let mut heap = BinaryHeap::new();
    heap.push(State::start(is_ultra));

    // initialize the dists
    let max_time = if is_ultra { 11 } else { 3 };
    let mut dists: HashMap<(Point, char, usize), usize> =
        (0..grid.nrows)
        .cartesian_product(0..grid.ncols)
        .cartesian_product(['.', '^', 'v', '<', '>'])
        .cartesian_product(0..max_time + 1)
        .map(|x| ((Point::new(x.0.0.0, x.0.0.1), x.0.1, x.1), usize::MAX))
        .collect();
    dists.insert(State::start(is_ultra).key(), 0);

    // define the target
    let target = Point::new(grid.nrows - 1, grid.ncols - 1);

    while let Some(state) = heap.pop() {
        // we've arrived at the target!
        if (state.loc == target) && state.allowed_to_stop() {
            if show_path{ show(grid, &state); }
            return state.cost;
        }

        // handle new history
        let mut prev = vec![(state.loc, state.dir)];
        if !state.prev.is_empty() { prev.extend(state.prev.iter()); }

        // add neighbors to the heap
        for neighbor in state.neighbors(grid) {
            let next = State::new(
                state.cost + grid.getnum(&neighbor),
                neighbor,
                prev.clone(),
                is_ultra,
            );
            if next.cost < dists[&next.key()] {
                dists.insert(next.key(), next.cost);
                heap.push(next);
            }
        }
    }
    0
}


fn show(grid: &Grid, state: &State) {
    let mut grid = grid.clone();
    for (p, dir) in state.prev.iter() {
        grid.cells[p.r][p.c] = *dir;
    }
    println!("{}", grid.to_string());
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    loc: Point,
    prev: Vec<(Point, char)>,
    dir: char,
    time_in_dir: usize,
    is_ultra: bool,
}

impl State {
    fn new(cost: usize, loc: Point, prev: Vec<(Point, char)>, is_ultra: bool) -> State {
        let (dir, time_in_dir) = if prev.is_empty() {
            ('.', 0)
        } else {
            let diff = (loc.r as i32 - prev[0].0.r as i32, loc.c as i32 - prev[0].0.c as i32);
            let dir = match diff {
                (-1, 0) => '^',
                (1, 0) => 'v',
                (0, -1) => '<',
                (0, 1) => '>',
                _ => '.',
            };
            let mut time_in_dir = 1_usize;
            for (_, d) in prev.iter() {
                if *d == dir { // ) || (*d == '.')
                    time_in_dir += 1;
                    continue;
                }
                break;
            };
            (dir, time_in_dir)
        };
        State{ cost, loc, prev, dir, time_in_dir, is_ultra }
    }

    /// Start in top left corner
    fn start(is_ultra: bool) -> State {
        State::new(0, Point::new(0, 0), vec![], is_ultra)
    }

    /// Key for position, including ability to move horizontal or vertical
    fn key(&self) -> (Point, char, usize) {
        (self.loc, self.dir, self.time_in_dir)
    }

    /// Is the crucible allowed to stop?
    fn allowed_to_stop(&self) -> bool {
        if !self.is_ultra { true } else { self.time_in_dir >= 4 }
    }

    /// Valid neighbors - ensuring we don't go off-grid or go more than 3 steps in one direction
    fn neighbors(&self, grid: &Grid) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        let is_moving_vertical = (self.dir == '^') || (self.dir == 'v');
        let is_moving_horizontal = (self.dir == '<') || (self.dir == '>');

        let (allow_vertical, allow_horizontal) = if self.prev.is_empty() {
            // handle start conditions
            (true, true)
        } else if !self.is_ultra {
            // handle normal crucible logic
            (
                !(is_moving_vertical && self.time_in_dir >= 3),
                !(is_moving_horizontal && self.time_in_dir >= 3),
            )
        } else {
            // handle ultra crucible logic
            (
                (is_moving_horizontal && (self.time_in_dir >= 4)) || (is_moving_vertical && (self.time_in_dir < 10)),
                (is_moving_vertical && (self.time_in_dir >= 4)) || (is_moving_horizontal && (self.time_in_dir < 10)),
            )
        };
        // construct the neighbors
        if allow_vertical {
            if self.loc.r > 0 { neighbors.push(self.loc.up(1)); }
            if self.loc.r < grid.nrows - 1 { neighbors.push(self.loc.down(1)); }
        };
        if allow_horizontal {
            if self.loc.c > 0 { neighbors.push(self.loc.left(1)); }
            if self.loc.c < grid.ncols - 1 { neighbors.push(self.loc.right(1)); }
        };
        // don't go back on yourself!
        if !self.prev.is_empty() {
            neighbors = neighbors
                .into_iter()
                .filter(|p| *p != self.prev[0].0)
                .collect();
        }
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