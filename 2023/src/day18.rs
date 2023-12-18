// SO BLOODY LONG...maybe 4 hours or so, faffing with inequalities!
use itertools::Itertools;

use crate::utils::{Point, Grid};

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    println!("PART 1: {}", solve(content, false));
}


fn part2(content: &str) {
    println!("PART 2: {}", solve(content, true));
}


fn solve(content: &str, fixed: bool) -> usize {
    let plan = DigPlan::new(content, fixed);
    plan.execute()
}


#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: char,
    length: usize,
}

impl Instruction {
    fn new(line: &str, fixed: bool) -> Instruction {
        let parts = line.split_whitespace().collect_vec();
        if !fixed {
            // for part 1
            Instruction{
                dir: parts[0].chars().nth(0).unwrap(),
                length: parts[1].parse().unwrap(),
            }
        } else {
            // for part 2
            Instruction{
                dir: match parts[2].chars().nth(7).unwrap() {
                    '0' => 'R', '1' => 'D', '2' => 'L', '3' => 'U', _ => '.'
                },
                length: usize::from_str_radix(&parts[2][2..7], 16).unwrap(),
            }
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Edge {
    start: Point,
    stop: Point,  // inclusive
}

impl Edge {
    fn new(p1: Point, p2: Point) -> Edge {
        let mut points = vec![p1, p2];
        points.sort();
        Edge{ start: points[0], stop: points[1] }
    }

    fn is_horizontal(&self) -> bool { self.start.r == self.stop.r }
    fn is_vertical(&self) -> bool { self.start.c == self.stop.c }

    fn length(&self) -> usize {
        self.stop.r - self.start.r + self.stop.c - self.start.c
    }

    fn contains(&self, p: &Point) -> bool {
        (p.r >= self.start.r) && (p.r <= self.stop.r) &&
        (p.c >= self.start.c) && (p.c <= self.stop.c)
    }
}


#[derive(Debug, Clone, Copy)]
struct Box {
    start: Point,
    stop: Point, // non-inclusive
}

impl Box {
    fn new(start: Point, stop: Point) -> Box { Box{ start, stop } }

    /// Area of this box which is in the interior
    fn interior(&self, trench: &[Edge]) -> usize {
        let mut size = 0_usize;
        let nrows = self.stop.r - self.start.r;
        let ncols = self.stop.c - self.start.c;

        // top-left corner
        if self.is_inside(trench, &self.start) { size += 1; }

        // left edge
        if nrows >= 2 {
            if self.is_inside(trench, &self.start.down(1)) {
                size += nrows - 1;
            }
        }
        // top edge
        if ncols >= 2 {
            if self.is_inside(trench, &self.start.right(1)) {
                size += ncols - 1;
            }
        }
        // inside
        if (nrows >= 2) && (ncols >= 2) {
            if self.is_inside(trench, &self.start.down(1).right(1)) {
                size += (nrows - 1) * (ncols - 1);
            }
        }
        /*if size > 0 {
            println!("({} -> {}, {} -> {}) {}",
            self.start.r + 1,
            self.stop.r,
            self.start.c + 1,
            self.stop.c,
            size);
        }*/
        size
    }

    fn is_inside(&self, trench: &[Edge], p: &Point) -> bool {
        if trench.iter().any(|e| e.contains(p)) {
            // this point is in the trench itself
            return false;
        }
        // horizontal edges above
        let n_crossings = trench.into_iter()
            .filter(|e| e.is_horizontal())
            .filter(|e| e.start.r < p.r)  // is above
            .filter(|e| e.start.c <= p.c)
            .filter(|e| e.stop.c > p.c)  // IMPORTANT THIS IS <
            .count();

        n_crossings % 2 == 1
    }
}


#[derive(Debug, Clone)]
struct DigPlan {
    instructions: Vec<Instruction>,
}

impl DigPlan {
    fn new(content: &str, fixed: bool) -> DigPlan {
        DigPlan{
            instructions: content.split("\n")
                .map(|line| Instruction::new(line, fixed))
                .collect(),
        }
    }

    /// Execute the plan
    fn execute(&self) -> usize {
        let trench = self.dig_trench();
        //_show(&trench);
        let boxes = self.to_boxes(&trench);
        let trench_size: usize = trench.iter().map(|e| e.length()).sum();
        let interior_size: usize = boxes.iter().map(|b| b.interior(&trench)).sum();
        trench_size + interior_size
    }
    
    /// Collect a vector of edges
    fn dig_trench(&self) -> Vec<Edge> {
        let mid = 100_000_000;
        let mut loc = Point::new(mid, mid);
        let mut trench = vec![];
        for i in self.instructions.iter() {
            let next = match i.dir {
                'U' => { loc.up(i.length) },
                'D' => { loc.down(i.length) },
                'L' => { loc.left(i.length) },
                'R' => { loc.right(i.length) },
                _ => loc,
            };
            trench.push(Edge::new(loc, next));
            loc = next;
        }
        // make nicer coords
        let minr = trench.iter().map(|e| e.start.r).min().unwrap();
        let minc = trench.iter().map(|e| e.start.c).min().unwrap();
        trench.iter()
            .map(|e| Edge::new(
                Point::new(e.start.r - minr, e.start.c - minc),
                Point::new(e.stop.r - minr, e.stop.c - minc),
            ))
            .collect_vec()
    }

    fn to_boxes(&self, trench: &[Edge]) -> Vec<Box> {
        let rows: Vec<usize> =
            trench.iter()
            .filter(|e| e.is_horizontal())
            .map(|e| e.start.r)
            .sorted()
            .dedup()
            .collect();
        let cols: Vec<usize> =
            trench.iter()
            .filter(|e| e.is_vertical())
            .map(|e| e.start.c)
            .sorted()
            .dedup()
            .collect();
        (1..rows.len())
            .cartesian_product(1..cols.len())
            .map(|x| (
                Box::new(
                    Point::new(rows[x.0 - 1], cols[x.1 - 1]),
                    Point::new(rows[x.0], cols[x.1])
                )
            ))
            .collect()
    }
}


fn _show(trench: &[Edge]) {
    let maxr = trench.iter().map(|e| e.stop.r).max().unwrap();
    let maxc = trench.iter().map(|e| e.stop.c).max().unwrap();
    let cells = (0..maxr + 1)
        .map(
            |r| (0..maxc + 1)
            .map(
                |c|
                if trench.iter()
                    .any(|e| e.contains(&Point::new(r, c)))
                    { '#' } else { '.' }
            )
            .collect_vec()
        )
        .collect_vec();
    println!("{}", Grid::new(cells).to_string());
}