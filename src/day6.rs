use crate::util::grid::Grid;
use crate::util::point::{Point, UP};
use std::collections::HashSet;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> (Grid<u8>, Point) {
    let grid = Grid::parse(input);
    let p = grid.find(b'^').unwrap();
    (grid, p)
}

#[aoc(day6, part1)]
fn part1((grid, start): &(Grid<u8>, Point)) -> usize {
    let mut guard = *start;
    let mut visited = HashSet::new();
    let mut dir = UP;
    while grid.contains(guard) {
        visited.insert(guard);
        if grid.get(guard + dir).is_none_or(|&b| b != b'#') {
            guard += dir;
        } else {
            dir = dir.clockwise();
        }
    }
    visited.len()
}

fn is_loop(grid: &Grid<u8>, start: Point) -> bool {
    let mut visited = HashSet::new();
    let mut dir = UP;
    let mut guard = start;
    while grid.contains(guard) {
        if !visited.insert((guard, dir)) {
            return true;
        }
        if grid.get(guard + dir).is_none_or(|&b| b != b'#') {
            guard += dir;
        } else {
            dir = dir.clockwise();
        }
    }
    false
}

#[aoc(day6, part2)]
fn part2((grid, start): &(Grid<u8>, Point)) -> usize {
    let mut grid = grid.clone();
    let mut res = 0;
    for p in grid.points() {
        if grid[p] != b'.' {
            continue;
        }

        grid[p] = b'#';
        if is_loop(&grid, *start) {
            res += 1;
        }
        grid[p] = b'.';
    }
    res
}
