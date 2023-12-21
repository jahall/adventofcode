#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub r: usize,
    pub c: usize,
}

impl Point {
    pub fn new(r: usize, c: usize) -> Point { Point{ r, c } }
    pub fn up(&self, inc: usize) -> Point { Point{ r: self.r - inc, c: self.c } }
    pub fn down(&self, inc: usize) -> Point { Point{ r: self.r + inc, c: self.c } }
    pub fn left(&self, inc: usize) -> Point { Point{ r: self.r, c: self.c - inc } }
    pub fn right(&self, inc: usize) -> Point { Point{ r: self.r, c: self.c + inc } }

    pub fn direct_neighbors(&self, grid: &Grid) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];
        if self.r > 0 { neighbors.push(self.up(1)); }
        if self.c > 0 { neighbors.push(self.left(1)); }
        if self.r < grid.nrows - 1 { neighbors.push(self.down(1)); }
        if self.c < grid.ncols - 1 { neighbors.push(self.right(1)); }
        neighbors
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.r, self.c)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub nrows: usize,
    pub ncols: usize,
}

pub enum GridRotation {
    Flip,
    Left,
    Right,
}

impl Grid {
    /// New grid from grid of chars
    pub fn new(cells: Vec<Vec<char>>) -> Grid {
        let nrows = cells.len();
        let ncols = cells[0].len();
        Grid { cells, nrows, ncols }
    }

    /// New grid from input string
    pub fn from_string(content: &str) -> Grid {
        let cells: Vec<Vec<char>> = content.split("\n")
            .map(|line| line.trim().chars().collect())
            .collect();
        Grid::new(cells)
    }

    /// New grid from concatenating other grids - used in day 21 for testing
    pub fn replicate(&self, shape: (usize, usize)) -> Grid {
        let mut cells = vec![];
        for _ in 0..shape.0 {
            let mut row_cells: Vec<Vec<char>> = vec![vec![]; self.nrows];
            for _ in 0..shape.1 {
                for (i, row) in self.cells.iter().enumerate() {
                    row_cells[i].extend(row.iter());
                }
            }
            cells.extend(row_cells);
        }
        Grid::new(cells)
    }

    pub fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|row| row.into_iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get(&self, p: &Point) -> &char {
        &self.cells[p.r][p.c]
    }

    pub fn getnum(&self, p: &Point) -> usize {
        self.get(p).to_digit(10).unwrap() as usize
    }

    pub fn rotate(&self, dir: GridRotation) -> Grid {
        let mut rotated = match dir {
            GridRotation::Flip => self.cells.clone(),
            _ => vec![vec!['.'; self.nrows]; self.ncols],
        };
        for r in 0..self.nrows {
            for c in 0..self.ncols {
                let (newr, newc) = match dir {
                    GridRotation::Flip => { (self.nrows - r - 1, self.ncols - c - 1) },
                    GridRotation::Left => { (self.ncols - c - 1, r) },
                    GridRotation::Right => { (c, self.nrows - r - 1) },
                };
                rotated[newr][newc] = self.cells[r][c];
            }
        }
        Grid::new(rotated)
    }
}