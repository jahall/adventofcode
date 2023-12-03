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
}