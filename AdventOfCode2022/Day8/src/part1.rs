// Part 1 functions

fn tree_is_on_outside(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    if row == 0 || row == vec.len() - 1 || col == 0 || col == vec[0].len() - 1 {
        return true;
    }
    false
}

fn is_tree_visible_by_row(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let cur_int = vec[row][col];
    // Assume it is visible unless we find a number that is greater than it on that side
    let mut visible_from_left = true;
    let mut visible_from_right = true;
    for i in 0..vec[row].len() {
        if i == col {
            continue;
        }
        // Check to the left of current int
        if i < col {
            if vec[row][i] >= cur_int {
                visible_from_left = false;
            }
        }
        // Check to the right of current int
        if i > col {
            if vec[row][i] >= cur_int {
                visible_from_right = false;
            }
        }
    }
    // true if visible from left or right (or both)
    visible_from_left || visible_from_right
}

fn is_tree_visible_by_column(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let cur_int = vec[row][col];
    // Assume it is visible unless we find a number that is greater than it on that side
    let mut visible_from_top = true;
    let mut visible_from_bottom = true;
    for i in 0..vec.len() {
        if i == row {
            continue;
        }
        // Check above current int
        if i < row {
            if vec[i][col] >= cur_int {
                visible_from_top = false;
            }
        }
        // Check below current int
        if i > row {
            if vec[i][col] >= cur_int {
                visible_from_bottom = false;
            }
        }
    }
    // true if visible from top or bottom (or both)
    visible_from_top || visible_from_bottom
}

pub fn get_num_visible_trees(vec: &Vec<Vec<i32>>) -> i32 {
    let mut num_visible_trees = 0;
    for row in 0..vec.len() {
        for col in 0..vec[row].len() {
            if tree_is_on_outside(vec, row, col) {
                num_visible_trees += 1;
                continue;
            }
            if is_tree_visible_by_row(vec, row, col) || is_tree_visible_by_column(vec, row, col) {
                num_visible_trees += 1;
            }
        }
    }
    num_visible_trees
}