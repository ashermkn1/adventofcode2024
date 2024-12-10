use crate::util::grid::Grid;
use crate::util::point::Point;
use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    grid: Grid<u8>,
    antennae: HashMap<u8, Vec<Point>>,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    let grid = Grid::parse(input);
    let mut antennae = HashMap::new();
    for p in grid.points() {
        let freq = grid[p];
        if freq != b'.' {
            antennae.entry(freq).or_insert_with(Vec::new).push(p);
        }
    }
    Input { grid, antennae }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut antinodes = input.grid.same_size_with(0);

    for antennae in input.antennae.values() {
        for &a in antennae {
            for &b in antennae {
                if a != b {
                    let dist = b - a;
                    let antinode = b + dist;
                    if antinodes.contains(antinode) {
                        antinodes[antinode] = 1;
                    }
                }
            }
        }
    }
    antinodes.bytes.iter().sum()
}

#[aoc(day8, part2)]
fn part2(input: &Input) -> usize {
    let mut antinodes = input.grid.same_size_with(0);

    for antennae in input.antennae.values() {
        for &a in antennae {
            for &b in antennae {
                if a != b {
                    let dist = b - a;
                    let mut antinode = b;
                    while antinodes.contains(antinode) {
                        antinodes[antinode] = 1;
                        antinode += dist;
                    }
                }
            }
        }
    }
    antinodes.bytes.iter().sum()
}
