use std::collections::HashMap;

#[derive(Clone)]
struct Stones {
    counts: HashMap<u64, usize>,
}

impl Stones {
    fn total(&self) -> usize {
        self.counts.values().sum()
    }
    fn step(&mut self) {
        let mut next = HashMap::new();

        for (&engraved, &count) in self.counts.iter() {
            if engraved == 0 {
                *next.entry(1).or_default() += count
            } else if (engraved.ilog10() + 1) % 2 == 0 {
                let split = 10u64.pow((engraved.ilog10() + 1) / 2);

                // left half
                *next.entry(engraved / split).or_default() += count;
                // right half
                *next.entry(engraved % split).or_default() += count;
            } else {
                *next.entry(engraved * 2024).or_default() += count;
            }
        }

        self.counts = next;
    }
    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Stones {
    let mut counts = HashMap::new();
    for num in input.split_ascii_whitespace() {
        let num = num.parse().unwrap();
        *counts.entry(num).or_default() += 1;
    }
    Stones { counts }
}

#[aoc(day11, part1)]
fn part1(input: &Stones) -> usize {
    let mut stones = input.clone();
    stones.step_n(25);
    stones.total()
}

#[aoc(day11, part2)]
fn part2(input: &Stones) -> usize {
    let mut stones = input.clone();
    stones.step_n(75);
    stones.total()
}
