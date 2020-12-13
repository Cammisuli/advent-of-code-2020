use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let adaptors = inputs();
    println!("part one: {}", part_one(&adaptors));
    println!("part one: {}", part_two(&adaptors));
}

fn part_one(adapters: &[isize]) -> isize {
    let (ones, threes) = adapters
        .iter()
        .tuple_windows()
        .fold((1, 1), |(ones, threes), (a, b)| match b - a {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => unreachable!(),
        });

    ones * threes
}

fn part_two(adapters: &[isize]) -> isize {
    let mut cache = HashMap::new();
    cache.insert(0, 1);
    for &i in adapters {
        let ans = cache.get(&(i - 1)).unwrap_or(&0)
            + cache.get(&(i - 2)).unwrap_or(&0)
            + cache.get(&(i - 3)).unwrap_or(&0);
        cache.insert(i, ans);
        println!("{}", ans);
    }
    cache[adapters.last().unwrap()]
}

fn inputs() -> Vec<isize> {
    let mut numbers = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<isize>>();
    numbers.sort();
    numbers
}
