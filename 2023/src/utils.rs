#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Point {
    pub r: usize,
    pub c: usize,
}

impl Point {
    pub fn new(r: usize, c: usize) -> Point { Point{ r, c } }
    pub fn up(&self) -> Point { Point{ r: self.r - 1, c: self.c } }
    pub fn down(&self) -> Point { Point{ r: self.r + 1, c: self.c } }
    pub fn left(&self) -> Point { Point{ r: self.r, c: self.c - 1 } }
    pub fn right(&self) -> Point { Point{ r: self.r, c: self.c + 1 } }
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
    pub fn new(cells: Vec<Vec<char>>) -> Grid {
        let nrows = cells.len();
        let ncols = cells[0].len();
        Grid { cells, nrows, ncols }
    }

    pub fn from_string(content: &str) -> Grid {
        let cells: Vec<Vec<char>> = content.split("\n")
            .map(|line| line.trim().chars().collect())
            .collect();
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