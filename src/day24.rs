use crate::day24::Operator::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type Wires = HashMap<String, bool>;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Operator {
    AND,
    OR,
    XOR,
}

impl Operator {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            AND => lhs && rhs,
            OR => lhs || rhs,
            XOR => lhs != rhs,
        }
    }
}
#[derive(Clone, Debug)]
struct Gate {
    in1: String,
    in2: String,
    out: String,
    op: Operator,
}

impl Gate {
    fn try_run(&self, wires: &Wires) -> Option<bool> {
        match (wires.get(&self.in1), wires.get(&self.in2)) {
            (Some(&b1), Some(&b2)) => Some(self.op.apply(b1, b2)),
            _ => None,
        }
    }
}
#[derive(Clone, Debug)]
struct System {
    wires: Wires,
    gates: Vec<Gate>,
}

impl System {
    fn simulate(&mut self) {
        let mut todo = VecDeque::from(self.gates.clone());
        while let Some(gate) = todo.pop_front() {
            if let Some(res) = gate.try_run(&self.wires) {
                self.wires.insert(gate.out, res);
            } else {
                todo.push_back(gate);
            }
        }
    }
    fn output(self) -> u64 {
        let bits = self
            .wires
            .into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .sorted_by_key(|(k, _)| k.to_owned())
            .rev()
            .map(|(_, v)| v);
        let mut res = 0;
        for bit in bits {
            res <<= 1;
            res |= bit as u64;
        }
        res
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> System {
    let (initial, gates) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    for line in initial.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        let value = value.parse::<u8>().map(|x| x == 1).unwrap();
        wires.insert(name.to_string(), value);
    }
    let gates = gates
        .lines()
        .map(|line| {
            let (in1, op, in2, _, out) = line.split_ascii_whitespace().collect_tuple().unwrap();
            let op = match op {
                "AND" => AND,
                "OR" => OR,
                "XOR" => XOR,
                _ => unreachable!(),
            };
            Gate {
                in1: in1.to_string(),
                in2: in2.to_string(),
                out: out.to_string(),
                op,
            }
        })
        .collect_vec();
    System { wires, gates }
}

#[aoc(day24, part1)]
fn part1(input: &System) -> u64 {
    let mut system = input.clone();
    system.simulate();
    system.output()
}

#[aoc(day24, part2)]
fn part2(System { gates, .. }: &System) -> String {
    // We use the fact that this system is a ripple adder
    let mut swapped = HashSet::with_capacity(8);
    let mut wire_gate = HashSet::new();
    for Gate { in1, in2, op, .. } in gates {
        wire_gate.insert((in1.clone(), *op));
        wire_gate.insert((in2.clone(), *op));
    }

    for Gate { in1, in2, out, op } in gates {
        match op {
            // Each AND should output to an OR, except the half-adder output
            AND => {
                if in1 != "x00" && in2 != "x00" && !wire_gate.contains(&(out.clone(), OR)) {
                    swapped.insert(out);
                }
            }
            OR => {
                // The only output that OR is allowed to be connected to is z45
                if out.starts_with('z') && out != "z45" {
                    swapped.insert(out);
                }
                // ORs go to carry out, which are AND/XOR and so should never point to
                // another OR
                if wire_gate.contains(&(out.clone(), OR)) {
                    swapped.insert(out);
                }
            }
            XOR => {
                // XORs come in two levels, the first level is between xs and ys
                // these all point to other XORs except the first one
                if in1.starts_with('x') || in2.starts_with('x') {
                    if in1 != "x00" && in2 != "x00" && !wire_gate.contains(&(out.clone(), XOR)) {
                        swapped.insert(out);
                    }
                } else if !out.starts_with('z') {
                    // The second layer of XORs must go to output
                    swapped.insert(out);
                }
            }
        }
    }
    swapped.into_iter().sorted().join(",")
}
