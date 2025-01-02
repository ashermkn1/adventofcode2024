use crate::util::point::Point;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};

#[derive(Clone)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

impl Machine {
    fn min_tokens(&self) -> Option<u64> {
        // calculate inverse of matrix
        // ax bx
        // ay by
        let det = self.a.x * self.b.y - self.b.x * self.a.y;
        if det == 0 {
            // either no solution or infinitely many
            // infinitely many if prize is in span of buttons
            // but thankfully this never happens!
            None
        } else {
            // unique solution
            // may not be integer
            let a = self.b.y * self.prize.x - self.b.x * self.prize.y;
            let b = self.a.x * self.prize.y - self.a.y * self.prize.x;
            if a % det != 0 || b % det != 0 {
                // solution is non-integer
                None
            } else {
                let a = a / det;
                let b = b / det;
                if a < 0 || b < 0 {
                    None
                } else {
                    Some((a * 3 + b) as u64)
                }
            }
        }
    }
}
#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let (_, [ax, ay, bx, by, px, py]) = caps.extract();
            Machine {
                a: Point::new(ax.parse().unwrap(), ay.parse().unwrap()),
                b: Point::new(bx.parse().unwrap(), by.parse().unwrap()),
                prize: Point::new(px.parse().unwrap(), py.parse().unwrap()),
            }
        })
        .collect_vec()
}

#[aoc(day13, part1)]
fn part1(machines: &[Machine]) -> u32 {
    machines
        .iter()
        .map(|machine| {
            let mut min_tokens = None;
            let max_a = min(
                100,
                min(machine.prize.x / machine.a.x, machine.prize.y / machine.a.y),
            );
            let max_b = min(
                100,
                min(machine.prize.x / machine.b.x, machine.prize.y / machine.b.y),
            );

            for a in 0..=max_a {
                for b in 0..=max_b {
                    if machine.a * a + machine.b * b == machine.prize {
                        let tokens = 3 * a + b;
                        if let Some(t) = min_tokens {
                            min_tokens = min(min_tokens, Some(tokens))
                        } else {
                            min_tokens = Some(tokens)
                        }
                    }
                }
            }
            min_tokens.unwrap_or(0) as u32
        })
        .sum()
}

#[aoc(day13, part2)]
fn part2(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .cloned()
        .map(|machine| Machine {
            prize: machine.prize + Point::new(10000000000000, 10000000000000),
            ..machine
        })
        .filter_map(|machine| machine.min_tokens())
        .sum()
}
