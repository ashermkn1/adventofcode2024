use crate::util::grid::Grid;
use crate::util::point::{Point, LEFT, RIGHT};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone)]
struct Input {
    map: Grid<u8>,
    moves: Vec<Point>,
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Input {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let map = Grid::parse(map);
    let moves = moves
        .bytes()
        .filter(|&c| c != b'\n')
        .map(Point::from)
        .collect_vec();

    Input { map, moves }
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i64 {
    let Input { mut map, moves } = input.clone();
    let mut robot = map.find(b'@').unwrap();

    for dir in moves {
        match map[robot + dir] {
            b'.' => {
                map[robot] = b'.';
                map[robot + dir] = b'@';
            }
            b'O' => {
                let mut curr = robot + dir;
                while map[curr] == b'O' {
                    curr += dir;
                }
                if map[curr] == b'#' {
                    // cannot make move
                    continue;
                }

                while curr != robot {
                    map[curr] = map[curr - dir];
                    curr -= dir;
                }
                map[robot] = b'.';
            }
            _ => continue,
        }
        robot += dir;
    }

    let mut res = 0;
    for p in map.points() {
        if map[p] == b'O' {
            res += p.x + 100 * p.y;
        }
    }
    res
}

fn upsize(map: Grid<u8>) -> Grid<u8> {
    let mut bytes = vec![];
    for b in map.bytes {
        match b {
            b'#' => {
                bytes.push(b'#');
                bytes.push(b'#');
            }
            b'O' => {
                bytes.push(b'[');
                bytes.push(b']');
            }
            b'.' => {
                bytes.push(b'.');
                bytes.push(b'.');
            }
            b'@' => {
                bytes.push(b'@');
                bytes.push(b'.');
            }
            _ => unreachable!(),
        }
    }
    Grid {
        width: map.width * 2,
        height: map.height,
        bytes,
    }
}

fn moving_blocks(map: &Grid<u8>, robot: Point, dir: Point) -> Option<HashSet<Point>> {
    let mut moving = HashSet::new();
    let mut frontier = vec![robot];
    let mut visited = HashSet::new();

    while let Some(p) = frontier.pop() {
        if !visited.insert(p) {
            continue;
        }
        match map[p] {
            b'#' => return None,
            b'.' => continue,
            b'@' => {
                moving.insert(p);
                frontier.push(p + dir)
            }
            b'[' => {
                moving.insert(p);
                frontier.push(p + RIGHT);
                frontier.push(p + dir);
            }
            b']' => {
                moving.insert(p);
                frontier.push(p + LEFT);
                frontier.push(p + dir);
            }
            _ => unreachable!(),
        }
    }

    Some(moving)
}
#[aoc(day15, part2)]
fn part2(input: &Input) -> i64 {
    let Input { map, moves } = input.clone();

    let mut map = upsize(map);
    let mut robot = map.find(b'@').unwrap();

    for dir in moves {
        match map[robot + dir] {
            b'.' => {
                map[robot + dir] = b'@';
                map[robot] = b'.';
            }
            b'[' | b']' => {
                if let Some(blob) = moving_blocks(&map, robot, dir) {
                    // put in descending order of manhattan distance to robot
                    // this ensures that we never overwrite a point that still needs to be moved
                    for &p in blob.iter().sorted_by_key(|p| p.manhattan(robot)).rev() {
                        map[p + dir] = map[p];
                        map[p] = b'.';
                    }
                } else {
                    continue;
                }
            }
            _ => continue,
        }
        robot += dir;
    }

    let mut res = 0;
    for p in map.points() {
        if map[p] == b'[' {
            res += p.x + 100 * p.y;
        }
    }
    res
}
