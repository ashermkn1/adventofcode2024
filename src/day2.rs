use itertools::{any, Itertools};

type Report = Vec<u32>;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect()
}

fn is_safe(report: &Report) -> bool {
    let mut up = true;
    let mut down = true;
    let mut range = true;
    for (&a, &b) in report.iter().tuple_windows() {
        up &= a < b;
        down &= a > b;
        range &= (1..=3).contains(&b.abs_diff(a))
    }
    (up ^ down) && range
}
#[aoc(day2, part1)]
fn part1(input: &[Report]) -> usize {
    input.iter().filter(|&report| is_safe(report)).count()
}

#[aoc(day2, part2)]
fn part2(input: &[Report]) -> usize {
    input
        .iter()
        .filter(|&report| {
            any(report.iter().copied().combinations(report.len() - 1), |x| {
                is_safe(&x)
            })
        })
        .count()
}
