use itertools::Itertools;
use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let numbers = inputs();
    let part_one = part_one(&numbers);
    //675280050
    println!("{}", part_one);
    println!("{}", part_two(part_one, &numbers));
}

fn inputs() -> Vec<usize> {
    include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(numbers: &[usize]) -> usize {
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

fn part_two(target: usize, numbers: &[usize]) -> impl Display {
    let (mut i, mut j, mut sum) = (0, 0, 0);
    while sum != target {
        if sum < target {
            sum += numbers[j];
            j += 1;
        } else {
            sum -= numbers[i];
            i += 1;
        }
    }
    let min = numbers[i..j].iter().min().unwrap();
    let max = numbers[i..j].iter().max().unwrap();
    min + max
}
