extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "file.pest"]
pub struct MemoryParser;

fn main() {
    let file = MemoryParser::parse(Rule::file, include_str!("input.txt"))
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    println!("part one: {}", part_one(file.clone()).unwrap());
    println!("part two: {}", part_two(file.clone()).unwrap());
}

fn part_one(file: Pair<Rule>) -> Option<usize> {
    let mut mem = HashMap::new();
    let mut mask: &[u8] = b"";
    for memories in file.into_inner() {
        if let Rule::memory = memories.as_rule() {
            for memory in memories.into_inner() {
                match memory.as_rule() {
                    Rule::mem => {
                        let mut memory_values = memory.into_inner();
                        let addr: usize = memory_values.next()?.as_str().parse().ok()?;
                        let mut value: usize = memory_values.next()?.as_str().parse().ok()?;

                        for (i, c) in mask.iter().rev().enumerate() {
                            match c {
                                b'0' => value &= !(1 << i),
                                b'1' => value |= 1 << i,
                                _ => {}
                            }
                        }
                        mem.insert(addr, value);
                    }
                    Rule::mask => {
                        mask = memory.as_str().as_bytes();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    Some(mem.values().sum())
}

fn part_two(file: Pair<Rule>) -> Option<usize> {
    let mut mem = HashMap::new();
    let mut mask: &[u8] = b"";
    for memories in file.into_inner() {
        if let Rule::memory = memories.as_rule() {
            for memory in memories.into_inner() {
                if let Rule::mem = memory.as_rule() {}
                match memory.as_rule() {
                    Rule::mem => {
                        let mut memory_values = memory.into_inner();
                        let addr: usize = memory_values.next()?.as_str().parse().ok()?;
                        let value: usize = memory_values.next()?.as_str().parse().ok()?;
                        write(&mut mem, mask, value, addr as isize, 0)
                    }
                    Rule::mask => {
                        mask = memory.as_str().as_bytes();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    Some(mem.values().sum())
}

fn write(mem: &mut HashMap<isize, usize>, mask: &[u8], value: usize, addr: isize, i: usize) {
    let bit = 1 << (35isize - i as isize).abs();
    match mask.get(i) {
        Some(b'0') => write(mem, mask, value, addr, i + 1),
        Some(b'1') => write(mem, mask, value, addr | bit, i + 1),
        Some(b'X') => {
            write(mem, mask, value, addr, i + 1);
            write(mem, mask, value, addr ^ bit, i + 1);
        }
        _ => {
            mem.insert(addr, value);
        }
    }
}
