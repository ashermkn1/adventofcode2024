use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let Some((left, right)) = line.split_once("   ") else {
                unreachable!()
            };
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .unzip()
}

#[aoc(day1, part1)]
fn part1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = (
        left.iter().copied().sorted().collect_vec(),
        right.iter().copied().sorted().collect_vec(),
    );
    left.into_iter()
        .zip(right)
        .map(|(l, r)| r.abs_diff(l))
        .sum()
}

#[aoc(day1, part2)]
fn part2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let freqs = right.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    left.iter().map(|x| x * freqs.get(x).unwrap_or(&0)).sum()
}
