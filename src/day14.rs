use crate::util::point::Point;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

const W: i64 = 101;
const H: i64 = 103;
#[derive(Copy, Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn step(&mut self) {
        let Point { x, y } = self.pos + self.vel;

        self.pos = Point::new(x.rem_euclid(W), y.rem_euclid(H));
    }

    fn step_n(&mut self, n: usize) {
        for _ in 0..n {
            self.step()
        }
    }
}
#[aoc_generator(day14)]
fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();
            let p = p.strip_prefix("p=").unwrap();
            let v = v.strip_prefix("v=").unwrap();

            let pos = p
                .split_once(',')
                .map(|(x, y)| {
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Point::new(x, y)
                })
                .unwrap();
            let vel = v
                .split_once(',')
                .map(|(x, y)| {
                    let x = x.parse().unwrap();
                    let y = y.parse().unwrap();
                    Point::new(x, y)
                })
                .unwrap();
            Robot { pos, vel }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &[Robot]) -> usize {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for mut robot in input.iter().copied() {
        robot.step_n(100);
        match (robot.pos.x.cmp(&(W / 2)), robot.pos.y.cmp(&(H / 2))) {
            (Ordering::Less, Ordering::Less) => q1 += 1,
            (Ordering::Greater, Ordering::Less) => q2 += 1,
            (Ordering::Less, Ordering::Greater) => q3 += 1,
            (Ordering::Greater, Ordering::Greater) => q4 += 1,
            _ => {}
        }
    }
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
fn part2(input: &[Robot]) -> u32 {
    let mut robots = input.iter().copied().collect_vec();
    for t in 1.. {
        let mut positions = HashSet::with_capacity(robots.len());
        for robot in &mut robots {
            robot.step();
            positions.insert(robot.pos);
        }
        if positions.len() == robots.len() {
            return t;
        }
    }
    0
}
