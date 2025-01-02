use crate::util::grid::Grid;
use crate::util::point::{DOWN, ORTHOGONAL, RIGHT};
use std::collections::HashSet;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse(input)
}

#[aoc(day12, part1)]
fn part1(input: &Grid<u8>) -> usize {
    // which cells are part of a region
    let mut visited = HashSet::with_capacity(input.bytes.len());
    let mut price = 0;
    for p in input.points() {
        if visited.contains(&p) {
            continue;
        }

        let crop = input[p];
        // find region containing p
        let mut region = HashSet::new();
        let mut stack = Vec::new();
        let mut perimeter = 0;
        stack.push(p);

        while let Some(p) = stack.pop() {
            if region.contains(&p) {
                continue;
            }
            region.insert(p);
            visited.insert(p);
            for dir in ORTHOGONAL {
                let next = p + dir;
                if !input.contains(next) || input[next] != crop {
                    // this direction crosses the region boundary
                    // so it is on the perimeter
                    perimeter += 1;
                } else if !region.contains(&next) {
                    stack.push(next);
                }
            }
        }
        price += region.len() * perimeter
    }

    price
}

#[aoc(day12, part2)]
fn part2(input: &Grid<u8>) -> usize {
    // which cells are part of a region
    let mut visited = HashSet::with_capacity(input.bytes.len());
    let mut price = 0;
    for p in input.points() {
        if visited.contains(&p) {
            continue;
        }

        let crop = input[p];
        // find region containing p
        let mut region = HashSet::new();
        let mut stack = Vec::new();
        let mut perimeter = HashSet::new();
        stack.push(p);

        while let Some(p) = stack.pop() {
            if region.contains(&p) {
                continue;
            }
            region.insert(p);
            visited.insert(p);
            for dir in ORTHOGONAL {
                let next = p + dir;
                if input.contains(next) && input[next] == crop {
                    stack.push(next);
                } else {
                    perimeter.insert((p, next));
                }
            }
        }

        // restricting our perimeter edges to ones that
        // are no longer perimeters shifted down/right gives us
        // one edge per side
        let mut sides = HashSet::new();
        for &(p1, p2) in &perimeter {
            if [RIGHT, DOWN]
                .into_iter()
                .map(|p| (p1 + p, p2 + p))
                .all(|p| !perimeter.contains(&p))
            {
                sides.insert((p1, p2));
            }
        }
        price += region.len() * sides.len()
    }

    price
}
