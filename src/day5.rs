use itertools::Itertools;
use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};

struct Input {
    before: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Input {
    fn is_correct(&self, update: &[u32]) -> bool {
        let mut seen = HashSet::new();
        let cell = OnceCell::new();
        for &page in update {
            let rule = self
                .before
                .get(&page)
                .unwrap_or(cell.get_or_init(HashSet::new));
            if !seen.is_subset(rule) {
                return false;
            }
            seen.insert(page);
        }
        true
    }
    fn correct_updates(&self) -> Vec<&[u32]> {
        self.updates
            .iter()
            .filter(|&update| self.is_correct(update))
            .map(|x| x.as_slice())
            .collect_vec()
    }
    fn incorrect_updates(&self) -> Vec<&[u32]> {
        self.updates
            .iter()
            .filter(|&update| !self.is_correct(update))
            .map(|x| x.as_slice())
            .collect_vec()
    }
}
#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in rules.lines() {
        let (before, after) = line
            .split_once('|')
            .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
            .unwrap();
        map.entry(after).or_default().insert(before);
    }
    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    Input {
        before: map,
        updates,
    }
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> u32 {
    let correct = input.correct_updates();
    correct.iter().map(|&update| update[update.len() / 2]).sum()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> u32 {
    let mut incorrect: Vec<Vec<u32>> = input
        .incorrect_updates()
        .iter()
        .map(|s| s.to_vec())
        .collect();
    let mut res = 0;
    for update in incorrect.iter_mut() {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(update.len() - 1) {
                if input.before[&update[i]].contains(&update[i + 1]) {
                    update.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
        res += update[update.len() / 2];
    }
    res
}
