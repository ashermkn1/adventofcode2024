use crate::util::point::{Point, ORIGIN};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::{once, repeat_n};

type Input = (Vec<(String, usize)>, Paths);
type Paths = HashMap<(char, char), HashSet<String>>;

fn gen_paths(paths: &mut Paths, keypad: &[(char, Point)], gap: Point) {
    for &(key1, src) in keypad {
        for &(key2, dest) in keypad {
            let horizontal = || {
                let c = if src.x < dest.x { '>' } else { '<' };
                repeat_n(c, dest.x.abs_diff(src.x) as usize)
            };
            let vertical = || {
                let c = if src.y < dest.y { 'v' } else { '^' };
                repeat_n(c, dest.y.abs_diff(src.y) as usize)
            };

            if Point::new(dest.x, src.y) != gap {
                let path = horizontal().chain(vertical()).chain(once('A')).collect();
                paths.entry((key1, key2)).or_default().insert(path);
            }

            if Point::new(src.x, dest.y) != gap {
                let path = vertical().chain(horizontal()).chain(once('A')).collect();
                paths.entry((key1, key2)).or_default().insert(path);
            }
        }
    }
}
#[aoc_generator(day21)]
fn parse_input(input: &str) -> Input {
    let codes = input
        .lines()
        .map(str::to_owned)
        .zip(
            input
                .lines()
                .map(|line| line.strip_suffix("A").unwrap().parse().unwrap()),
        )
        .collect();
    let num_gap = Point::new(0, 3);
    let num_keypad = [
        ('7', Point::new(0, 0)),
        ('8', Point::new(1, 0)),
        ('9', Point::new(2, 0)),
        ('4', Point::new(0, 1)),
        ('5', Point::new(1, 1)),
        ('6', Point::new(2, 1)),
        ('1', Point::new(0, 2)),
        ('2', Point::new(1, 2)),
        ('3', Point::new(2, 2)),
        ('0', Point::new(1, 3)),
        ('A', Point::new(2, 3)),
    ];
    let dir_keypad = [
        ('^', Point::new(1, 0)),
        ('A', Point::new(2, 0)),
        ('<', Point::new(0, 1)),
        ('v', Point::new(1, 1)),
        ('>', Point::new(2, 1)),
    ];

    let mut paths = HashMap::new();
    gen_paths(&mut paths, &num_keypad, num_gap);
    gen_paths(&mut paths, &dir_keypad, ORIGIN);
    (codes, paths)
}

type Cache = HashMap<(char, char, usize), usize>;
fn dfs(paths: &Paths, cache: &mut Cache, code: &str, depth: usize) -> usize {
    if depth == 0 {
        return code.len();
    }

    let mut prev_key = 'A';
    let mut res = 0;

    for c in code.chars() {
        let memo = (prev_key, c, depth);

        res += cache.get(&memo).copied().unwrap_or_else(|| {
            let num_presses = paths[&(prev_key, c)]
                .iter()
                .map(|path| dfs(paths, cache, path, depth - 1))
                .min()
                .unwrap();
            cache.insert(memo, num_presses);
            num_presses
        });
        prev_key = c;
    }
    res
}

fn chained_robots((codes, paths): &Input, depth: usize) -> usize {
    let cache = &mut HashMap::with_capacity(1000);
    codes
        .iter()
        .map(|(code, num)| dfs(paths, cache, code, depth) * num)
        .sum()
}
#[aoc(day21, part1)]
fn part1(input: &Input) -> usize {
    chained_robots(input, 3)
}

#[aoc(day21, part2)]
fn part2(input: &Input) -> usize {
    chained_robots(input, 26)
}
