use std::str::FromStr;

use itertools::Itertools;

struct Schematic {
    is_key: bool,
    heights: [u8; 5],
}

impl Schematic {
    fn fits(&self, other: &Schematic) -> bool {
        if self.is_key == other.is_key {
            return false;
        }
        self.heights
            .iter()
            .zip(other.heights)
            .all(|(&a, b)| a + b <= 7)
    }
}
impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|l| l.as_bytes()).collect_vec();
        let is_key = lines[0][0] == b'.';
        let mut heights = [0; 5];
        for line in lines {
            for i in 0..line.len() {
                if line[i] == b'#' {
                    heights[i] += 1;
                }
            }
        }
        Ok(Schematic { is_key, heights })
    }
}

#[aoc(day25, part1)]
fn part1(input: &str) -> u32 {
    let schematics = input
        .split("\n\n")
        .map(|x| Schematic::from_str(x).unwrap())
        .collect_vec();
    let mut res = 0;
    let (keys, locks): (Vec<_>, Vec<_>) = schematics.into_iter().partition(|s| s.is_key);
    for key in keys {
        for lock in &locks {
            if key.fits(lock) {
                res += 1;
            }
        }
    }
    res
}

#[aoc(day25, part2)]
fn part2(_: &str) -> u32 {
    0
}
