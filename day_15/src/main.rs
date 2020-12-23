use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let inputs = [1, 0, 15, 2, 10, 13];

    println!("part one: {}", speakers(&inputs, 2020));
    println!("part two: {}", speakers(&inputs, 30000000));
}

fn speakers(starting_numbers: &[u32], total: usize) -> usize {
    let mut seen = starting_numbers
        .iter()
        .enumerate()
        .map(|(i, &num)| (num as usize, i + 1))
        .collect::<HashMap<_, _>>();

    (starting_numbers.len() + 1..total).fold(0, |last, i| i - seen.insert(last, i).unwrap_or(i))
}
