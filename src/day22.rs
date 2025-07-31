use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
#[derive(Debug, Copy, Clone)]
struct Secret(u64);

impl Secret {
    fn mix(&self, val: u64) -> Self {
        Secret(self.0 ^ val)
    }
    fn prune(&self) -> Self {
        Secret(self.0 % 16777216)
    }

    fn evolve(&self) -> Self {
        let val = self.0 * 64;
        let step1 = self.mix(val).prune();
        let val = step1.0 / 32;
        let step2 = step1.mix(val).prune();
        let val = step2.0 * 2048;
        step2.mix(val).prune()
    }

    fn price(&self) -> u64 {
        self.0 % 10
    }

    fn iter(&self) -> SecretIter {
        SecretIter {
            secret: *self,
            steps: 0,
        }
    }
}

struct SecretIter {
    secret: Secret,
    steps: usize,
}

impl Iterator for SecretIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 2000 {
            None
        } else {
            let price = self.secret.price();
            self.secret = self.secret.evolve();
            self.steps += 1;
            Some(price as i32)
        }
    }
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> Vec<Secret> {
    input
        .lines()
        .map(|line| Secret(line.parse().unwrap()))
        .collect()
}

#[aoc(day22, part1)]
fn part1(input: &[Secret]) -> u64 {
    input
        .par_iter()
        .copied()
        .map(|secret| {
            let mut curr = secret;
            for _ in 0..2000 {
                curr = curr.evolve();
            }
            curr.0
        })
        .sum()
}

#[aoc(day22, part2)]
fn part2(initials: &[Secret]) -> i32 {
    let mut bananas = HashMap::new();
    for &initial in initials {
        let mut combinations = HashSet::new();
        for (p0, p1, p2, p3, p4) in initial.iter().tuple_windows() {
            let key = (p1 - p0, p2 - p1, p3 - p2, p4 - p3);
            // once per buyer
            if combinations.insert(key) {
                bananas
                    .entry(key)
                    .and_modify(|total| *total += p4)
                    .or_insert(p4);
            }
        }
    }
    bananas.into_values().max().unwrap()
}
