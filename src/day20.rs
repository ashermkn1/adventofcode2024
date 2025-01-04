use crate::util::grid::Grid;
use crate::util::point::{Point, ORTHOGONAL};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;

type Input = (Grid<u8>, Grid<u32>);
#[aoc_generator(day20)]
fn parse_input(input: &str) -> Input {
    let input = Grid::parse(input);
    let start = input.find(b'S').unwrap();

    let mut frontier = VecDeque::new();
    frontier.push_back((start, 0));
    let mut visited = input.same_size_with(false);
    visited[start] = true;
    let mut dist = input.same_size_with(u32::MAX);

    while let Some((p, steps)) = frontier.pop_front() {
        dist[p] = steps;
        for dir in ORTHOGONAL {
            let next = p + dir;
            if input.contains(next) && input[next] != b'#' && !visited[next] {
                visited[next] = true;
                frontier.push_back((next, steps + 1))
            }
        }
    }
    (input, dist)
}

#[aoc(day20, part1)]
fn part1((input, dist): &Input) -> usize {
    let mut res = 0;
    for p in input.points().filter(|&p| input[p] != b'#') {
        for step1 in ORTHOGONAL {
            for step2 in ORTHOGONAL {
                let exit = p + step1 + step2;
                if input.contains(exit) && dist[p] + 102 <= dist[exit] {
                    res += 1
                }
            }
        }
    }

    res
}

#[aoc(day20, part2)]
fn part2((_, dist): &Input) -> usize {
    let points = dist.points().filter(|&p| dist[p] != u32::MAX).collect_vec();

    points
        .into_par_iter()
        .map(|p| {
            let mut cheats = 0;
            // check everything purely to the right
            cheats += (2..=20)
                .filter(|&dx| good_cheat(dist, p, Point::new(dx, 0)))
                .count();
            // now check everything below with manhattan distance <= 20
            for dy in 1..=20 {
                for dx in (dy - 20)..=(20 - dy) {
                    cheats += good_cheat(dist, p, Point::new(dx, dy)) as usize;
                }
            }
            cheats
        })
        .sum()
}

fn good_cheat(dist: &Grid<u32>, start: Point, delta: Point) -> bool {
    let end = start + delta;
    dist.contains(end)
        && dist[end] != u32::MAX
        && (dist[end].abs_diff(dist[start]) - (end.manhattan(start) as u32) >= 100)
}
