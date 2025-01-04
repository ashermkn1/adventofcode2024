use std::collections::{HashMap, HashSet};

struct Input {
    towels: HashSet<String>,
    designs: Vec<String>,
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Input {
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(str::to_owned).collect();
    let designs = designs.lines().map(str::to_owned).collect();
    Input { towels, designs }
}

fn is_valid_design(
    design: &str,
    towels: &HashSet<String>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if cache.contains_key(design) {
        return cache[design];
    }

    if design.is_empty() {
        return true;
    }

    for towel in towels {
        if let Some(trunc) = design.strip_prefix(towel) {
            if is_valid_design(trunc, towels, cache) {
                cache.insert(design.to_owned(), true);
                return true;
            }
        }
    }
    cache.insert(design.to_owned(), false);
    false
}

#[aoc(day19, part1)]
fn part1(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input
        .designs
        .iter()
        .filter(|design| is_valid_design(design, &input.towels, &mut cache))
        .count()
}

fn ways_to_make(
    design: &str,
    towels: &HashSet<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if cache.contains_key(design) {
        return cache[design];
    }

    if design.is_empty() {
        return 1;
    }

    let mut ways = 0;
    for towel in towels {
        if let Some(trunc) = design.strip_prefix(towel) {
            ways += ways_to_make(trunc, towels, cache);
        }
    }
    cache.insert(design.to_owned(), ways);
    ways
}

#[aoc(day19, part2)]
fn part2(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input
        .designs
        .iter()
        .map(|design| ways_to_make(design, &input.towels, &mut cache))
        .sum()
}
