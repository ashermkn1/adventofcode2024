use crate::util::grid::Grid;
use crate::util::point::{Point, ORTHOGONAL};
use std::collections::{HashSet, VecDeque};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn get_score(grid: &Grid<u8>, start: Point) -> u32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut score = 0;
    queue.push_back(start);

    while let Some(p) = queue.pop_front() {
        if visited.insert(p) && grid[p] == b'9' {
            score += 1;

            continue;
        }

        for dir in ORTHOGONAL {
            let next = p + dir;
            if grid.contains(next) && !visited.contains(&next) && grid[next] == grid[p] + 1 {
                queue.push_back(next);
            }
        }
    }

    score
}

#[aoc(day10, part1)]
fn part1(grid: &Grid<u8>) -> u32 {
    let mut total = 0;
    for p in grid.points() {
        if grid[p] == b'0' {
            let score = get_score(grid, p);
            total += score;
        }
    }
    total
}

fn get_rating(grid: &Grid<u8>, start: Point) -> u32 {
    let mut queue = VecDeque::new();
    let mut rating = 0;
    queue.push_back(start);

    while let Some(p) = queue.pop_front() {
        if grid[p] == b'9' {
            rating += 1;
            continue;
        }

        for dir in ORTHOGONAL {
            let next = p + dir;
            if grid.contains(next) && grid[next] == grid[p] + 1 {
                queue.push_back(next);
            }
        }
    }

    rating
}

#[aoc(day10, part2)]
fn part2(grid: &Grid<u8>) -> u32 {
    let mut total = 0;
    for p in grid.points() {
        if grid[p] == b'0' {
            let score = get_rating(grid, p);
            total += score;
        }
    }
    total
}
