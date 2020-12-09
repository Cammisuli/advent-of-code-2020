use itertools::Itertools;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let numbers = inputs();

    //675280050
    println!("{}", part_one(&numbers));
}

fn inputs() -> Vec<usize> {
    include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(numbers: &[usize]) -> impl Display {
    numbers
        // windows takes a length, while numbers is a index
        .windows(26)
        .find(|nums| {
            nums.iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != nums[25])
        })
        .map(|nums| nums[25])
        .unwrap()
}
