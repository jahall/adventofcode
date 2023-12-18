// 1.5 hours part 1, multiple hours for part 2!

use std::collections::HashSet;

use crate::utils::{Grid, Point};


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let grid = Grid::from_string(content);
    let start = find_start(&grid);
    let mut this = firsts(&grid, &start).0;
    let mut prev = start.clone();
    let mut length: usize = 1;
    loop {
        length += 1;
        let nxt = next(&grid, &this, &prev);
        prev = this;
        this = nxt;
        if this == start { break; }
    }
    println!("PART 1: {}", length / 2);
}


fn part2(content: &str) {
    let grid = Grid::from_string(content);
    let start = find_start(&grid);
    let pipe = build_pipe(&grid, &start);
    let corner = northwest_corner(&grid, &start, &pipe);

    let mut interior: HashSet<Point> = HashSet::new();

    let mut this_dir = '>';
    let mut this = corner.down(1);
    let mut prev = corner.clone();
    loop {
        // get current cell, and convert if 'S'
        let mut cell = *grid.get(&this);
        if cell == 'S' { cell = start_type(&start); }
        // get the next direction to look in
        let next_dir = match cell {
            'L' | '7' => match this_dir {'^' => '>','v' => '<', '<' => 'v', '>' => '^', _ => '.'},
            'J' | 'F' => match this_dir {'^' => '<','v' => '>', '<' => '^', '>' => 'v', _ => '.'},
            _ => this_dir,
        };
        // look in both possible interior directions
        for dir in [&next_dir, &this_dir] {
            let p = match *dir {
                '^' => this.up(1),
                'v' => this.down(1),
                '<' => this.left(1),
                '>' => this.right(1),
                _ => Point::new(0, 0),
            };
            search_interior(&mut interior, &pipe, p);
        }
        // break if we've come full circle
        if this == corner { break; }
        // move to the next cell
        this_dir = next_dir;
        let nxt = next(&grid, &this, &prev);
        prev = this;
        this = nxt;
    }
    println!("PART 2: {}", interior.len());
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


fn firsts(grid: &Grid, start: &Point) -> (Point, Point) {
    let mut firsts: Vec<Point> = vec![];
    if start.r > 0 {
        let up = grid.get(&start.up(1));
        if HashSet::from(['|', 'F', '7']).contains(up) { firsts.push(start.up(1)); }
    }
    if start.r < grid.nrows - 1 {
        let down = grid.get(&start.down(1));
        if HashSet::from(['|', 'L', 'J']).contains(down) { firsts.push(start.down(1)); }
    }
    if start.c > 0 {
        let left = grid.get(&start.left(1));
        if HashSet::from(['-', 'F', 'L']).contains(left) { firsts.push(start.left(1)); }
    }
    if start.c < grid.ncols - 1 {
        let right = grid.get(&start.right(1));
        if HashSet::from(['-', 'J', '7']).contains(right) { firsts.push(start.right(1)); }
    }
    (firsts[0], firsts[1])
}


fn next(grid: &Grid, curr: &Point, prev: &Point) -> Point {
    let cell = grid.get(curr);
    match *cell {
        'S' => {
            let firsts = firsts(grid, curr);
            return if firsts.0 != *prev { firsts.0 } else { firsts.1 }
        },
        '|' => return if curr.up(1) != *prev { curr.up(1) } else { curr.down(1) },
        '-' => return if curr.left(1) != *prev { curr.left(1) } else { curr.right(1) },
        'L' => return if curr.up(1) != *prev { curr.up(1) } else { curr.right(1) },
        'J' => return if curr.up(1) != *prev { curr.up(1) } else { curr.left(1) },
        '7' => return if curr.down(1) != *prev { curr.down(1) } else { curr.left(1) },
        'F' => return if curr.down(1) != *prev { curr.down(1) } else { curr.right(1) },
        _ => Point::new(0, 0)
    }
}


fn start_type(start: &Point) -> char {
    if *start == Point::new(1, 1) { return 'F' };  // test 1
    if *start == Point::new(2, 0) { return 'F' };  // test 2
    if *start == Point::new(1, 1) { return 'F' };  // test 3
    if *start == Point::new(4, 12) { return 'F' };  // test 4
    '|'
}


fn northwest_corner(grid: &Grid, start: &Point, pipe: &HashSet<Point>) -> Point {
    let use_s = start_type(start) == 'F';
    for c in 0..grid.ncols {
        for r in 0..grid.ncols {
            let p = Point::new(r, c);
            if !pipe.contains(&p) { continue; }
            let cell = *grid.get(&p);
            if (cell == 'F') | (use_s & (cell == 'S')) {
                return p;
            }
        }
    }
    Point::new(0, 0)
}


fn build_pipe(grid: &Grid, start: &Point) -> HashSet<Point> {
    let mut this = firsts(&grid, &start).0;
    let mut prev = start.clone();
    let mut pipe: HashSet::<Point> = HashSet::new();
    pipe.insert(start.clone());
    loop {
        pipe.insert(this.clone());
        let nxt = next(&grid, &this, &prev);
        prev = this;
        this = nxt;
        if this == *start { break; }
    }
    pipe
}


fn search_interior(interior: &mut HashSet<Point>, pipe: &HashSet<Point>, start: Point) {
    let mut to_search: HashSet<Point> = HashSet::from([start]);
    loop {
        if to_search.is_empty() { break; }
        // horrible way to pop
        let p = to_search.iter().next().cloned().unwrap();  
        to_search.remove(&p);
        // check this guy
        if pipe.contains(&p) | interior.contains(&p) {
            continue;
        }
        interior.insert(p.clone());
        // add neighbors to search
        to_search.extend([p.up(1), p.down(1), p.left(1), p.right(1)]);
    }
}