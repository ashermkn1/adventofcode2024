use crate::util::grid::Grid;
use crate::util::point::{Point, DIAGONAL, ORTHOGONAL};
use itertools::Itertools;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

fn search(grid: &Grid<u8>, p: Point) -> u32 {
    let next = |c| match c {
        b'X' => b'M',
        b'M' => b'A',
        b'A' => b'S',
        b'S' => b'S',
        _ => unreachable!(),
    };
    let dirs = [ORTHOGONAL, DIAGONAL]
        .iter()
        .copied()
        .flatten()
        .collect_vec();
    let mut res = 0;
    for dp in dirs {
        let mut curr = b'X';
        let mut good = true;
        for step in 0..=3 {
            if let Some(&c) = grid.get(p + dp * step) {
                if c == curr {
                    curr = next(curr);
                } else {
                    good = false;
                    break;
                }
            } else {
                good = false;
                break;
            }
        }
        if good {
            res += 1
        }
    }
    res
}
#[aoc(day4, part1)]
fn part1(input: &Grid<u8>) -> u32 {
    let mut res = 0;
    for row in 0..input.height {
        for col in 0..input.width {
            let p = Point::new(col, row);
            res += search(input, p)
        }
    }
    res
}

#[aoc(day4, part2)]
fn part2(grid: &Grid<u8>) -> u32 {
    let cross = |p| {
        let target = [Some(b'M'), Some(b'A'), Some(b'S')];
        let main_diag = [
            grid.get(p + DIAGONAL[0]),
            grid.get(p),
            grid.get(p + DIAGONAL[3]),
        ]
        .map(|x| x.copied());
        if main_diag == target
            || (main_diag.to_vec() == target.iter().copied().rev().collect::<Vec<_>>())
        {
            let opp_diag = [
                grid.get(p + DIAGONAL[1]),
                grid.get(p),
                grid.get(p + DIAGONAL[2]),
            ]
            .map(|x| x.copied());
            if opp_diag == target
                || (opp_diag.to_vec() == target.iter().copied().rev().collect_vec())
            {
                return true;
            }
        }
        false
    };
    let mut res = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if cross(Point::new(col, row)) {
                res += 1;
            }
        }
    }
    res
}
