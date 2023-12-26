// Maybe 1.5 hours for part 1...another hour for part 2 (but took 30 mins to run!)

use std::collections::{HashSet, VecDeque, HashMap};

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
    let grid = Grid::from_string(content);
    let graph = make_graph(&grid);
    println!("PART 2: {}", find_longest_graph_path(&grid, &graph));
}


/// Find longest path through the maze
fn find_longest_path(grid: &Grid) -> usize {
    let start = Point::new(0, 1);
    let mut queue = VecDeque::new();
    queue.push_back(Path::new(vec![start]));
    let mut longest = 0;
    while let Some(path) = queue.pop_front() {
        let path = path.move_to_decision(&grid);
        if path.finished(grid) && path.length() > longest {
            longest = path.length();
        }
        for next in path.valid_moves(grid) {
            queue.push_back(path.step(next));
        }
    }
    longest
}


/// Find longest path through the graph
fn find_longest_graph_path(grid: &Grid, graph: &HashMap<Point, HashMap<Point, usize>>) -> usize {
    let start = Point::new(0, 1);
    let mut queue = VecDeque::new();
    queue.push_back(GraphPath::new(start));
    let mut longest = 0;
    while let Some(path) = queue.pop_front() {
        if path.finished(grid) { println!("{}", path.length); }
        if path.finished(grid) && path.length > longest {
            longest = path.length;
            println!("{} <----", path.length);
        }
        for (nbr, edge) in graph[&path.node].iter() {
            if !path.set.contains(&nbr) {
                queue.push_back(path.step(nbr, *edge))
            }
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


#[derive(Debug, Clone)]
struct GraphPath {
    node: Point,
    set: HashSet<Point>,
    length: usize,
}

impl GraphPath {
    fn new(node: Point) -> GraphPath {
        GraphPath{ node, set: HashSet::from([node]), length: 0 }
    }

    /// Step to next node
    fn step(&self, next: &Point, edge: usize) -> GraphPath {
        let mut set = HashSet::from([*next]);
        set.extend(&self.set);
        GraphPath{ node: *next, set, length: self.length + edge }
    }

    /// Have we finished?
    fn finished(&self, grid: &Grid) -> bool {
        let finish = Point::new(grid.nrows - 1, grid.ncols - 2);
        self.node == finish
    }
}


/// Make a graph of key junctions
fn make_graph(grid: &Grid) -> HashMap<Point, HashMap<Point, usize>> {
    let nodes = (0..grid.nrows)
        .cartesian_product(0..grid.ncols)
        .map(|(r, c)| Point::new(r, c))
        .filter(|p| is_node(grid, p))
        .collect_vec();
    let mut graph: HashMap<Point, HashMap<Point, usize>> = HashMap::new();
    for node in nodes {
        for next in next_steps(grid, &node) {
            let (other, length) = walk(grid, &node, &next);
            graph.entry(node)
                .and_modify(|e| { e.insert(other, length); } )
                .or_insert(HashMap::from([(other, length)]));
        }
    }
    graph
}


/// Is this point a node in the graph?
fn is_node(grid: &Grid, point: &Point) -> bool {
    let source = Point::new(0, 1);
    let target = Point::new(grid.nrows - 1, grid.ncols - 2);
    if (*point == source) || (*point == target) {
        return true;
    }
    if (point.r == 0) || (point.r == grid.nrows - 1) || (point.c == 0) || (point.c == grid.ncols - 1) {
        return false;
    }
    if *grid.get(point) == '#' {
        return false;
    }
    let n_edges = point.direct_neighbors(grid)
        .iter()
        .filter(|p| *grid.get(p) != '#')
        .count();
    n_edges != 2
}


/// Walk from one node to the next
fn walk(grid: &Grid, node: &Point, next: &Point) -> (Point, usize) {
    let mut length = 0_usize;
    let mut prev = *node;
    let mut next = *next;
    loop {
        length += 1;
        if is_node(grid, &next) { break; }
        let nexts = next_steps(grid, &next);
        for nbr in nexts {
            if nbr != prev {
                prev = next;
                next = nbr;
                break;
            }
        }
    }
    (next, length)
}


/// Valid next steps
fn next_steps(grid: &Grid, point: &Point) -> Vec<Point> {
    point.direct_neighbors(grid)
        .iter()
        .filter(|p| *grid.get(p) != '#')
        .map(|p| *p)
        .collect()
}


fn _show(grid: &Grid, path: &Path) {
    let mut grid = grid.clone();
    for p in path.set.iter() {
        grid.cells[p.r][p.c] = 'o';
    }
    println!("{}", grid.to_string());
}