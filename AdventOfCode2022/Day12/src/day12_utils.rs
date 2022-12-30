use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GridPos {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct GridPosWithCost {
    pub cost: u32,
    pub pos: GridPos,
}
impl Ord for GridPosWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for GridPosWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_grid_neighbours(grid: &Vec<Vec<char>>, cur_pos: &GridPos) -> Vec<GridPos> {
    let mut neighbours: Vec<GridPos> = Vec::new();
    // Up
    if cur_pos.row > 0 {
        neighbours.push(GridPos {
            row: cur_pos.row - 1,
            col: cur_pos.col,
        });
    }
    // Down
    if cur_pos.row < grid.len() - 1 {
        neighbours.push(GridPos {
            row: cur_pos.row + 1,
            col: cur_pos.col,
        });
    }
    // Left
    if cur_pos.col > 0 {
        neighbours.push(GridPos {
            row: cur_pos.row,
            col: cur_pos.col - 1,
        });
    }
    // Right
    if cur_pos.col < grid[0].len() - 1 {
        neighbours.push(GridPos {
            row: cur_pos.row,
            col: cur_pos.col + 1,
        });
    }
    neighbours
}
