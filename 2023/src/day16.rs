// Maybe an hour for part 1, then 10 mins for part 2

use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

use crate::utils::{Grid, Point};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    let start = Beam::new(0, 0, '>');
    let energised = run_contraption(&grid, start, false);
    println!("PART 1: {}", energised);
}


fn part2(content: &str) {
    let grid = Grid::from_string(content);
    let mut starts: Vec<Beam> = vec![];
    starts.extend((0..grid.ncols).map(|c| Beam::new(0, c, 'v')).collect_vec());
    starts.extend((0..grid.ncols).map(|c| Beam::new(grid.nrows - 1, c, '^')).collect_vec());
    starts.extend((0..grid.nrows).map(|r| Beam::new(r, 0, '>')).collect_vec());
    starts.extend((0..grid.nrows).map(|r| Beam::new(r, grid.ncols - 1, '<')).collect_vec());
    let energised: usize = starts.iter()
        .map(|start| run_contraption(&grid, *start, false))
        .max()
        .unwrap();
    println!("PART 2: {}", energised);
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Beam{
    loc: Point,
    dir: char,
}

impl Beam {
    fn new(r: usize, c: usize, dir: char) -> Beam {
        Beam{ loc: Point::new(r, c), dir }
    }

    /// Change location
    fn change_loc(&self, loc: Point) -> Beam {
        Beam{ loc, dir: self.dir }
    }

    /// Change direction
    fn change_dir(&self, dir: char) -> Beam {
        Beam{ loc: self.loc, dir }
    }

    /// Move the beam a step in the direction its facing, or None if it falls off the grid
    fn mv(&self, grid: &Grid) -> Option<Beam> {
        if
            ((self.dir == '^') & (self.loc.r == 0)) |
            ((self.dir == 'v') & (self.loc.r == grid.nrows - 1)) |
            ((self.dir == '<') & (self.loc.c == 0)) |
            ((self.dir == '>') & (self.loc.c == grid.ncols - 1))
        {
            return None;
        }
        Some(
            match self.dir {
                '^' => self.change_loc(self.loc.up()),
                'v' => self.change_loc(self.loc.down()),
                '<' => self.change_loc(self.loc.left()),
                '>' => self.change_loc(self.loc.right()),
                _ => *self,
            }
        )
    }

    /// Turn or split the beam based on the mirror in this cell
    fn turn(&self, grid: &Grid) -> Vec<Beam> {
        let mut beams: Vec<Beam> = vec![];
        let mirror = *grid.get(&self.loc);
        match (mirror, self.dir) {
            ('/', '^') | ('\\', 'v') => { beams.push(self.change_dir('>')) },
            ('/', 'v') | ('\\', '^') => { beams.push(self.change_dir('<')) },
            ('/', '<') | ('\\', '>') => { beams.push(self.change_dir('v')) },
            ('/', '>') | ('\\', '<') => { beams.push(self.change_dir('^')) },
            ('|', '<') | ('|', '>') => {
                beams.push(self.change_dir('^'));
                beams.push(self.change_dir('v'));
            },
            ('-', '^') | ('-', 'v') => {
                beams.push(self.change_dir('<'));
                beams.push(self.change_dir('>'));
            },
            _ => { beams.push(self.change_dir(self.dir)) },
        }
        beams
    }
}


/// Run the contraption, based on some starting beam
fn run_contraption(grid: &Grid, start: Beam, show: bool) -> usize {
    let mut energised: HashSet<Point> = HashSet::new();
    let mut active: VecDeque<Beam> = VecDeque::from([start]);
    let mut visited: HashSet<Beam> = HashSet::new();

    let mut display = grid.clone();
    loop {
        // Exit loop if we've exhausted all active beams
        if active.is_empty() { break; }

        // Pop the next active beam
        let beam = active.pop_front().unwrap();
        visited.insert(beam);
        energised.insert(beam.loc);

        // Turn the beam based on the mirror, and add new beams to the queue
        for next in beam.turn(&grid) {
            let next = next.mv(&grid);
            // note && is lazy whilst & is not
            if next.is_some() && !visited.contains(&next.unwrap()) {
                active.push_back(next.unwrap());
            }
        }

        // Update the display
        if show & (display.cells[beam.loc.r][beam.loc.c] == '.') {
            display.cells[beam.loc.r][beam.loc.c] = beam.dir;
        }
    }
    if show { println!("{}", display.to_string()); }
    energised.len()
}