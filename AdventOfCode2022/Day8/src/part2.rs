// Part 2 functions

fn num_trees_visible_left_and_right(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> (i32, i32) {
    let cur_int = vec[row][col];
    let mut left_trees = 0;
    let mut right_trees = 0;
    // Check all numbers to the left of current int
    for i in (0..col).rev() {
        // Can't start from col - 1, because usize will underflow
        // ... there is probably a better way though
        if i == col {
            continue;
        }
        if vec[row][i] < cur_int {
            left_trees += 1;
        } else {
            left_trees += 1;
            break;
        }
    }
    // Check all numbers to the right of current int
    for i in col..vec[row].len() {
        if i == col {
            continue;
        }
        if vec[row][i] < cur_int {
            right_trees += 1;
        } else {
            right_trees += 1;
            break;
        }
    }
    (left_trees, right_trees)
}

fn num_trees_visible_top_and_bottom(vec: &Vec<Vec<i32>>, row: usize, col: usize) -> (i32, i32) {
    let cur_int = vec[row][col];
    let mut top_trees = 0;
    let mut bottom_trees = 0;
    // Check all numbers above current int
    for i in (0..row).rev() {
        if i == row {
            continue;
        }
        if vec[i][col] < cur_int {
            top_trees += 1;
        } else {
            top_trees += 1;
            break;
        }
    }
    // Check all numbers below current int
    for i in row..vec.len() {
        if i == row {
            continue;
        }
        if vec[i][col] < cur_int {
            bottom_trees += 1;
        } else {
            bottom_trees += 1;
            break;
        }
    }
    (top_trees, bottom_trees)
}

pub fn get_highest_scenic_score(lines_as_ints: &Vec<Vec<i32>>) -> i32 {
    let mut cur_high_score = 0;
    for row in 0..lines_as_ints.len() {
        for col in 0..lines_as_ints[row].len() {
            let (left_trees, right_trees) =
                num_trees_visible_left_and_right(&lines_as_ints, row, col);
            let (top_trees, bottom_trees) =
                num_trees_visible_top_and_bottom(&lines_as_ints, row, col);
            let total_score = left_trees * right_trees * top_trees * bottom_trees;
            if total_score > cur_high_score {
                cur_high_score = total_score;
            }
        }
    }
    cur_high_score
}
