// Maybe 1.5 hours for part 1, 10 mins for (slow) part 2

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    println!("PART 1: {}", find_longest_path(&grid));
}


fn part2(content: &str) {
    let mut grid = Grid::from_string(content);
    // make all slopes normal
    for r in 0..grid.nrows {
        for c in 0..grid.ncols {
            let cell = *grid.get(&Point::new(r, c));
            if (cell != '#') && (cell != '.') { grid.cells[r][c] = '.'; }
        }
    }
    println!("PART 2: {}", find_longest_path(&grid));
}


/// Find longest path through the maze
fn find_longest_path(grid: &Grid) -> usize {
    let start = Point::new(0, 1);
    let mut queue = VecDeque::new();
    queue.push_back(Path::new(vec![start]));
    let mut longest = 0;
    while let Some(path) = queue.pop_front() {
        let path = path.move_to_decision(&grid);
        if path.finished(grid) { println!("{}", path.length()); }
        if path.finished(grid) && path.length() > longest {
            longest = path.length();
        }
        for next in path.valid_moves(grid) {
            queue.push_back(path.step(next));
        }
    }
    longest
}


#[derive(Debug, Clone)]
struct Path {
    steps: Vec<Point>,
    set: HashSet<Point>,
}

impl Path {
    fn new(steps: Vec<Point>) -> Path {
        Path{ steps: steps.clone(), set: HashSet::from_iter(steps) }
    }

    /// Make a new path by stepping in a direction
    fn step(&self, next: Point) -> Path {
        let mut new = self.clone();
        new.step_mut(next);
        new
    }

    /// Add a step to this path
    fn step_mut(&mut self, next: Point) {
        self.steps.push(next);
        self.set.insert(next);
    }

    /// Length of path
    fn length(&self) -> usize { self.steps.len() - 1 }

    /// What moves can we make?
    fn valid_moves(&self, grid: &Grid) -> Vec<Point> {
        let current = *self.steps.iter().last().unwrap();
        let mut moves = vec![];
        // up
        if current.r > 0 {
            let next = current.up(1);
            let c = *grid.get(&next);
            if (c == '.') || (c == '^') { moves.push(next); }
        }
        // down
        if current.r < grid.nrows - 1 {
            let next = current.down(1);
            let c = *grid.get(&next);
            if (c == '.') || (c == 'v') { moves.push(next); }
        }
        // left
        if current.c > 0 {
            let next = current.left(1);
            let c = *grid.get(&next);
            if (c == '.') || (c == '<') { moves.push(next); }
        }
        // right
        if current.c < grid.ncols - 1 {
            let next = current.right(1);
            let c = *grid.get(&next);
            if (c == '.') || (c == '>') { moves.push(next); }
        }
        moves
            .iter()
            .filter(|p| !self.set.contains(p))
            .map(|p| *p)
            .collect_vec()
    }

    /// Move as far as poss
    fn move_to_decision(&self, grid: &Grid) -> Path {
        let mut next = self.clone();
        loop {
            let moves = next.valid_moves(grid);
            if moves.len() != 1 { return next; }
            next = next.step(moves[0]);
        }
    }

    /// Have we finished?
    fn finished(&self, grid: &Grid) -> bool {
        let current = *self.steps.iter().last().unwrap();
        let finish = Point::new(grid.nrows - 1, grid.ncols - 2);
        current == finish
    }
}


fn show(grid: &Grid, path: &Path) {
    let mut grid = grid.clone();
    for p in path.set.iter() {
        grid.cells[p.r][p.c] = 'o';
    }
    println!("{}", grid.to_string());
}