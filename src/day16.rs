use crate::util::grid::Grid;
use crate::util::heap::MinHeap;
use crate::util::point::{Point, DOWN, LEFT, ORTHOGONAL, RIGHT, UP};
use std::cell::OnceCell;
use std::collections::{HashSet, VecDeque};

const DIRS: [Point; 4] = [RIGHT, DOWN, LEFT, UP];
#[aoc_generator(day16)]
fn parse_input(input: &str) -> (u32, usize) {
    let map = Grid::parse(input);
    let start = map.find(b'S').unwrap();
    let end = map.find(b'E').unwrap();
    let mut pq = MinHeap::new();
    let mut dist = map.same_size_with([u32::MAX; 4]);
    let mut min_score = u32::MAX;

    pq.push(0, (start, 0));
    dist[start][0] = 0;
    while let Some((score, (p, facing))) = pq.pop() {
        if score >= min_score {
            continue;
        }
        if p == end {
            min_score = score;
            continue;
        }

        let clockwise = (facing + 1) % 4;
        let cclockwise = (facing + 3) % 4;
        let next = [
            (score + 1, (p + DIRS[facing], facing)),
            (score + 1000, (p, clockwise)),
            (score + 1000, (p, cclockwise)),
        ];

        for (nscore, (next, ndir)) in next {
            if map[next] != b'#' && nscore < dist[next][ndir] {
                pq.push(nscore, (next, ndir));
                dist[next][ndir] = nscore;
            }
        }
    }

    let mut frontier = VecDeque::new();
    let mut on_best = map.same_size_with(false);

    for dir in 0..DIRS.len() {
        if dist[end][dir] == min_score {
            frontier.push_back((min_score, end, dir));
        }
    }

    while let Some((score, p, facing)) = frontier.pop_front() {
        on_best[p] = true;
        if p == start {
            continue;
        }

        let clockwise = (facing + 3) % 4;
        let cclockwise = (facing + 1) % 4;
        let next = [
            (score - 1, p - DIRS[facing], facing),
            (score - 1000, p, clockwise),
            (score - 1000, p, cclockwise),
        ];

        for state @ (nscore, next, ndir) in next {
            if nscore == dist[next][ndir] {
                frontier.push_back(state);
                dist[p][facing] = u32::MAX;
            }
        }
    }
    (min_score, on_best.bytes.into_iter().filter(|&b| b).count())
}

#[aoc(day16, part1)]
fn part1(answers: &(u32, usize)) -> u32 {
    answers.0
}

#[aoc(day16, part2)]
fn part2(answers: &(u32, usize)) -> usize {
    answers.1
}
