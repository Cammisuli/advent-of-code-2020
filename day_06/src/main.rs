use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let inputs = inputs();

    let part_one: usize = inputs
        .iter()
        .map(|group| group.chars().filter(|c| c.is_alphabetic()).unique().count())
        .sum();

    let part_two: usize = inputs
        .iter()
        .map(|group| {
            group
                .lines()
                .fold(('a'..='z').collect::<HashSet<_>>(), |acc, line| {
                    acc.intersection(&line.chars().collect::<HashSet<_>>())
                        .cloned()
                        .collect()
                })
                .len()
        })
        .sum();

    println!("Part one: {:?}", part_one);
    println!("Part two: {:?}", part_two);
    assert_eq!(part_one, 6170);
    assert_eq!(part_two, 2947);
}

fn inputs() -> Vec<String> {
    include_str!("input.txt")
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect()
}
