mod day12_utils;

use day12_utils::{get_grid_neighbours, GridPos, GridPosWithCost};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let part1 = solve_part1("Day12Puzzle.txt");
    println!("Part 1: {}", part1);
    let part2 = solve_part2("Day12Puzzle.txt");
    println!("Part 2: {}", part2);
}

fn make_grid_from_file(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect(&*format!("Unable to open file {}", filename));
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let row: Vec<char> = line.expect("Unable to read a line").chars().collect();
        grid.push(row);
    }
    grid
}

// Part 1
// Find the shortest path from the start ('S') to the end ('E')
fn solve_part1(filename: &str) -> u32 {
    let mut grid = make_grid_from_file(filename);
    let start_pos = find_start_position(&grid).expect("Unable to find start position");
    let end_pos = find_end_position(&grid).expect("Unable to find end position");
    let solution = dijkstra_solve(&mut grid, &start_pos, &end_pos);
    solution
}

// Part 2
// Find the shortest path from the start (any 'a' or 'S') to the end ('E')
// Takes a few seconds, but fine for this puzzle (release build is best)
fn solve_part2(filename: &str) -> u32 {
    let mut grid = make_grid_from_file(filename);
    let start_positions = find_all_a_positions(&grid);
    let end_pos = find_end_position(&grid).expect("Unable to find end position");
    let mut steps_for_each = Vec::new();
    for pos in start_positions {
        let steps = dijkstra_solve(&mut grid, &pos, &end_pos);
        // If steps is 0, then no path was found
        if steps != 0 {
            steps_for_each.push(steps);
        }
    }
    let min_steps = steps_for_each
        .iter()
        .min()
        .expect("Unable to find min steps");
    *min_steps
}

// Based off of Dijkstra algorithm
fn dijkstra_solve(grid: &mut Vec<Vec<char>>, start_pos: &GridPos, end_pos: &GridPos) -> u32 {
    let mut pos_cost_map = HashMap::<GridPos, u8>::new();
    // Add all the positions to the map with letter values (height) from 0 to 25
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let letter = match grid[row][col] {
                'S' => 'a',
                'E' => 'z',
                'a'..='z' => grid[row][col],
                _ => continue,
            };
            let value = letter as u8 - b'a';
            pos_cost_map.insert(GridPos { row, col }, value);
        }
    }

    let mut visited = HashSet::new();
    let mut priority_queue = BinaryHeap::new();

    priority_queue.push(GridPosWithCost {
        cost: 0,
        pos: *start_pos,
    });
    visited.insert(*start_pos);

    // While the queue is not empty
    while let Some(GridPosWithCost { pos, cost }) = priority_queue.pop() {
        // If we have reached the end position, return the distance
        if pos == *end_pos {
            return cost;
        }

        // Get the current position's height value from the map
        let cur_height = pos_cost_map
            .get(&pos)
            .expect("Unable to get height of position");

        // Get the neighbours of the current position and filter out invalid ones
        let neighbours = get_grid_neighbours(grid, &pos);
        let valid_neighbours = neighbours.into_iter().filter(|neighbour| {
            let neighbour_height = pos_cost_map
                .get(neighbour)
                .expect("Unable to get height of neighbour");
            *neighbour_height <= *cur_height || *neighbour_height == *cur_height + 1
        });

        // For each neighbour, add it to the queue if it has not already been visited
        for neighbour in valid_neighbours {
            if visited.insert(neighbour) {
                priority_queue.push(GridPosWithCost {
                    cost: cost + 1,
                    pos: neighbour,
                });
            }
        }
    }
    0
}

fn find_start_position(grid: &Vec<Vec<char>>) -> Option<GridPos> {
    // Only need to search the first column
    for i in 0..grid.len() {
        if grid[i][0] == 'S' {
            // Previous position is None because we're at the start
            return Some(GridPos { row: i, col: 0 });
        }
    }
    None
}

fn find_end_position(grid: &Vec<Vec<char>>) -> Option<GridPos> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'E' {
                return Some(GridPos { row: i, col: j });
            }
        }
    }
    None
}

fn find_all_a_positions(grid: &Vec<Vec<char>>) -> Vec<GridPos> {
    let mut positions = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                'S' => positions.push(GridPos { row: i, col: j }),
                'a' => positions.push(GridPos { row: i, col: j }),
                _ => continue,
            }
        }
    }
    positions
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle() {
        let mut grid = make_grid_from_file("Day12TestPuzzle.txt");
        let start_pos = find_start_position(&grid).expect("No start position found");
        let end_pos = find_end_position(&grid).expect("No end position found");
        let solution = dijkstra_solve(&mut grid, &start_pos, &end_pos);
        println!("Solution: {}", solution);
        assert_eq!(solution, 31);
    }
}
