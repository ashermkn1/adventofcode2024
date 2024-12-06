use regex::Regex;
use Instr::*;
enum Instr {
    Mul(u32),
    Do,
    Dont,
}

#[aoc_generator(day3, part1)]
fn parse1(input: &str) -> Vec<u32> {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul_re
        .captures_iter(input)
        .map(|m| {
            let a: u32 = m.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = m.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .collect()
}

#[aoc_generator(day3, part2)]
fn parse2(input: &str) -> Vec<Instr> {
    let re = Regex::new(r"do(?:n't)?\(\)|mul\(\d+,\d+\)").unwrap();
    re.captures_iter(input)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => Do,
            "don't()" => Dont,
            s => {
                let (a, b) = s
                    .strip_prefix("mul(")
                    .map(|s| s.strip_suffix(")"))
                    .unwrap()
                    .unwrap()
                    .split_once(',')
                    .unwrap();
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                Mul(a * b)
            }
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Instr]) -> u32 {
    input
        .iter()
        .fold((true, 0), |(good, acc), x| match x {
            Do => (true, acc),
            Dont => (false, acc),
            Mul(x) => (good, if good { acc + x } else { acc }),
        })
        .1
}
