use rayon::prelude::*;
use Op::*;
#[derive(Debug, Clone)]
struct Equation {
    test: u64,
    vals: Vec<u64>,
}

enum Op {
    Mul,
    Add,
    Concat,
}

impl Op {
    fn eval(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Mul => lhs * rhs,
            Add => rhs + lhs,
            Concat => lhs * 10u64.pow(rhs.ilog10() + 1) + rhs,
        }
    }
}
impl Equation {
    fn can_make(&self, part1: bool, curr: u64, i: usize) -> bool {
        if i >= self.vals.len() {
            return self.test == curr;
        }

        if part1 {
            [Mul, Add]
                .into_par_iter()
                .map(|op| self.can_make(part1, op.eval(curr, self.vals[i]), i + 1))
                .any(|x| x)
        } else {
            [Mul, Add, Concat]
                .into_par_iter()
                .map(|op| self.can_make(part1, op.eval(curr, self.vals[i]), i + 1))
                .any(|x| x)
        }
    }
}
#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (test, vals) = line.split_once(": ").unwrap();
            let test = test.parse().unwrap();
            let vals = vals
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            Equation { test, vals }
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Vec<Equation>) -> u64 {
    input
        .par_iter()
        .filter(|&eq| eq.can_make(true, eq.vals[0], 1))
        .map(|eq| eq.test)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &Vec<Equation>) -> u64 {
    input
        .par_iter()
        .filter(|&eq| eq.can_make(false, eq.vals[0], 1))
        .map(|eq| eq.test)
        .sum()
}
