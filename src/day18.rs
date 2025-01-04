use crate::util::grid::Grid;
use crate::util::point::{Point, ORIGIN, ORTHOGONAL};
use crate::util::union_find::UnionFind;
use std::collections::VecDeque;

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            Point::new(x, y)
        })
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Point]) -> u32 {
    let mut memory = Grid::new(71, 71, true);
    for &p in input.iter().take(1024) {
        memory[p] = false;
    }

    let start = Point::new(0, 0);
    let end = Point::new(70, 70);

    let mut frontier = VecDeque::new();
    let mut visited = memory.same_size_with(false);

    frontier.push_back((start, 0));

    while let Some((p, steps)) = frontier.pop_front() {
        if p == end {
            return steps;
        }

        for dir in ORTHOGONAL {
            let next = p + dir;
            if memory.contains(next) && memory[next] && !visited[next] {
                visited[next] = true;
                frontier.push_back((next, steps + 1))
            }
        }
    }
    0
}

#[aoc(day18, part2)]
fn part2(input: &[Point]) -> Point {
    let start = Point::new(0, 0);
    let end = Point::new(70, 70);
    let mut corrupted = Grid::new(71, 71, false);
    for &p in input {
        corrupted[p] = true;
    }

    // the block that ends up putting start and end in the same
    // connected component is our answer
    let mut uf = UnionFind::from_iter(corrupted.points());

    // compute connected components of all non-corrupted blocks
    for p in corrupted.points().filter(|&p| !corrupted[p]) {
        for q in ORTHOGONAL.map(|q| q + p) {
            if corrupted.contains(q) && !corrupted[q] {
                uf.union(p, q);
            }
        }
    }

    for &block in input.iter().rev() {
        for q in ORTHOGONAL.map(|q| q + block) {
            if corrupted.contains(q) && !corrupted[q] {
                uf.union(block, q);
            }
        }
        corrupted[block] = false;
        if uf.find(start) == uf.find(end) {
            return block;
        }
    }
    ORIGIN
}
