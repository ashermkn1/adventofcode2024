use itertools::Itertools;
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct State {
    a: u32,
    b: u32,
    c: u32,
}

impl State {
    fn combo_op(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn simulate(self, program: &[u32]) -> Vec<u32> {
        let mut state = self;

        let mut outputs = vec![];
        let mut pc = 0;

        while pc < program.len() {
            match program[pc] {
                // adv
                0 => {
                    let denom = 2u32.pow(state.combo_op(program[pc + 1]));
                    state.a /= denom;
                }
                // bxl
                1 => state.b ^= program[pc + 1],
                // bst
                2 => state.b = state.combo_op(program[pc + 1]) % 8,
                // jnz
                3 => {
                    if state.a != 0 {
                        pc = program[pc + 1] as usize;
                        continue;
                    }
                }
                // bxc
                4 => state.b ^= state.c,
                // out
                5 => outputs.push(state.combo_op(program[pc + 1]) % 8),
                // bdv
                6 => {
                    let denom = 2u32.pow(state.combo_op(program[pc + 1]));
                    state.b = state.a / denom;
                }
                // cdv
                7 => {
                    let denom = 2u32.pow(state.combo_op(program[pc + 1]));
                    state.c = state.a / denom;
                }
                _ => unreachable!("Invalid opcode"),
            }
            pc += 2;
        }
        outputs
    }
}
type Input = (State, Vec<u32>);
#[aoc_generator(day17)]
fn parse_input(input: &str) -> (State, Vec<u32>) {
    let (state, program) = input.split_once("\n\n").unwrap();
    let (a, b, c) = state
        .lines()
        .map(|line| {
            let (_, num) = line.split_once(": ").unwrap();
            num.parse().unwrap()
        })
        .collect_tuple()
        .unwrap();

    let program = program
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    (State { a, b, c }, program)
}

#[aoc(day17, part1)]
fn part1((state, program): &Input) -> String {
    let outputs = state.simulate(program);
    outputs.into_iter().join(",")
}

fn output(a: u64) -> u32 {
    let mut b = a % 8;
    b ^= 3;
    let c = a >> b;
    b ^= 4;
    b ^= c;
    (b % 8) as u32
}

#[aoc(day17, part2)]
fn part2((_, prog): &Input) -> u64 {
    // build up the quines 3 bits at a time
    let mut candidates = HashSet::new();
    candidates.insert(0);

    for num in prog.iter().copied().rev() {
        let mut next_cands = HashSet::new();
        for cand in candidates {
            for i in 0..8 {
                let x = (cand << 3) + i;
                if output(x) == num {
                    next_cands.insert(x);
                }
            }
        }
        candidates = next_cands
    }

    candidates.into_iter().min().unwrap()
}
