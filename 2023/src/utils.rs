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


#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub nrows: usize,
    pub ncols: usize,
}

impl Grid {
    pub fn new(content: &str) -> Grid {
        let cells: Vec<Vec<char>> = content.split("\n")
            .map(|line| line.trim().chars().collect())
            .collect();
        let nrows = cells.len();
        let ncols = cells[0].len();
        Grid { cells, nrows, ncols }
    }

    pub fn get(&self, p: &Point) -> &char {
        &self.cells[p.r][p.c]
    }
}