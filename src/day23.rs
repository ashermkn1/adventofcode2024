use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const NODES: usize = (26 * 25 + 25) + 1;

// adjacency list and adjacency matrix, best of both worlds
type Input = (HashMap<usize, Vec<usize>>, [[bool; NODES]; NODES]);

#[aoc_generator(day23)]
fn parse_input(input: &str) -> Input {
    let mut adjlist: HashMap<usize, Vec<_>> = HashMap::new();
    let mut adjmatrix = [[false; NODES]; NODES];
    for line in input.lines() {
        let (from, to) = line
            .split_once('-')
            .map(|(from, to)| (to_index(from.as_bytes()), to_index(to.as_bytes())))
            .unwrap();
        adjlist.entry(from).or_default().push(to);
        adjlist.entry(to).or_default().push(from);
        adjmatrix[from][to] = true;
        adjmatrix[to][from] = true;
    }
    (adjlist, adjmatrix)
}

#[aoc(day23, part1)]
fn part1((nbors, edges): &Input) -> u32 {
    let mut res = 0;
    let mut seen = [false; NODES];
    for node in to_index(b"ta")..=to_index(b"tz") {
        if let Some(list) = nbors.get(&node) {
            seen[node] = true;
            for (&x, &y) in list.iter().tuple_combinations() {
                if !seen[x] && !seen[y] && edges[x][y] {
                    res += 1;
                }
            }
        }
    }
    res
}

fn is_clique(nbors: &HashMap<usize, Vec<usize>>, clique: &[usize]) -> bool {
    for i in 0..clique.len() {
        for j in i + 1..clique.len() {
            if !nbors[&clique[i]].contains(&clique[j]) {
                return false;
            }
        }
    }
    true
}
#[aoc(day23, part2)]
fn part2((nbors, _): &Input) -> String {
    // From experimentation, we know that the graph is 13-regular
    // and there are no cliques of size 14, so we start with searching for ones
    // of size 13.
    for (&v, adj) in nbors {
        for &w in adj {
            let adj_set = nbors[&w].iter().copied().collect::<HashSet<_>>();
            let mut cand = vec![v, w];
            for &w2 in adj {
                if adj_set.contains(&w2) {
                    cand.push(w2);
                }
            }
            if cand.len() == 13 && is_clique(nbors, &cand) {
                return cand.into_iter().sorted().map(from_index).join(",");
            }
        }
    }
    unreachable!("We should find one!");
}

fn to_index(name: &[u8]) -> usize {
    let a = (name[0] - b'a') as usize;
    let b = (name[1] - b'a') as usize;
    26 * a + b
}

fn from_index(index: usize) -> String {
    let b = (index % 26) as u8;
    let a = (index / 26) as u8;
    String::from_utf8(Vec::from([b'a' + a, b'a' + b])).unwrap()
}
