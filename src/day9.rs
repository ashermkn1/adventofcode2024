use crate::util::parse::ToDigit;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
enum Block {
    Empty(usize),
    File(usize, usize),
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty(size) => {
                write!(f, "{}", ".".repeat(*size))
            }
            Block::File(id, size) => {
                if *id < 10 {
                    write!(f, "{}", id.to_string().repeat(*size))
                } else {
                    let mut s = id.to_string();
                    s.insert(0, '(');
                    s.push(')');
                    write!(f, "{}", s.repeat(*size))
                }
            }
        }
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Block> {
    let mut disk = vec![];
    let mut id = 0;

    let mut bytes = input.bytes();
    while let Some(file) = bytes.next() {
        if let Some(size) = file.to_digit() {
            disk.push(Block::File(id, size as usize));
            id += 1;
        }

        if let Some(empty) = bytes.next() {
            if let Some(size) = empty.to_digit() {
                disk.push(Block::Empty(size as usize));
            }
        }
    }
    disk
}

#[aoc(day9, part1)]
fn part1(input: &[Block]) -> usize {
    let mut res = 0;
    let mut right = input.len() - 1;

    let mut needed = 0;
    let mut right_id = 0;
    let mut position = 0;
    if let Block::File(id, size) = input[right] {
        needed = size;
        right_id = id;
    }

    for left in 0..input.len() {
        if left >= right {
            break;
        }
        match input[left] {
            Block::File(id, size) => {
                // use artithmetic mean
                // sum from position to position + size of i
                // all times id
                res += id * ((position * 2 + size - 1) * size / 2);
                position += size;
            }
            Block::Empty(size) => {
                let mut gap_size = size;

                while gap_size > 0 {
                    let take = usize::min(needed, gap_size);
                    needed -= take;
                    gap_size -= take;
                    res += right_id * ((position * 2 + take - 1) * take / 2);
                    position += take;
                    // we finished the file block
                    if needed == 0 {
                        right -= 2;
                        if right <= left {
                            break;
                        }
                        if let Block::File(id, size) = input[right] {
                            right_id = id;
                            needed = size;
                        }
                    }
                }
            }
        }
    }

    for _ in 0..needed {
        res += position * right_id;
        position += 1;
    }
    res
}

#[aoc(day9, part2)]
fn part2(input: &[Block]) -> usize {
    let mut free: Vec<_> = (0..10).map(|_| BinaryHeap::with_capacity(1_000)).collect();
    let mut pos = 0;
    for &block in input {
        match block {
            Block::Empty(size) => {
                free[size].push(Reverse(pos));
                pos += size;
            }
            Block::File(_, size) => pos += size,
        }
    }

    // free now contains each size of free blocks
    // in order of appearance
    let mut checksum = 0;
    for &block in input.iter().rev() {
        match block {
            Block::Empty(size) => {
                pos -= size;
            }
            Block::File(id, size) => {
                pos -= size;
                // find leftmost, smallest block that can fit the file
                let mut dest = pos;
                let mut size_used = usize::MAX;
                for (i, heap) in free.iter().enumerate().skip(size) {
                    if let Some(&Reverse(fpos)) = heap.peek() {
                        if fpos < dest {
                            dest = fpos;
                            size_used = i;
                        }
                    }
                }

                // update checksum
                checksum += id * ((dest * 2 + size - 1) * size / 2);

                if size_used != usize::MAX {
                    free[size_used].pop();
                    if size < size_used {
                        free[size_used - size].push(Reverse(dest + size));
                    }
                }
            }
        }
    }
    checksum
}
