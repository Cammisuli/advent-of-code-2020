extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "bags.pest"]
pub struct BagParser;

type BagContainers = HashMap<String, HashMap<String, usize>>;

fn main() {
    let inputs = inputs();
    let part_one = inputs
        .keys()
        .filter(|color| can_reach_gold_bag(&inputs, color))
        .count();

    let part_two = bag_count(&inputs, "shiny gold") - 1;

    println!("{:?}", part_one);
    println!("{:?}", part_two);

    let input = BagParser::parse(Rule::bags, include_str!("input.txt"));
    println!("{:?}", input);
}

fn inputs() -> BagContainers {
    let file = include_str!("input.txt");
    file.lines()
        .map(|line| {
            let mut parts = line.split(" bags contain ");
            let color = parts.next().unwrap().to_string();
            let rules = parts
                .next()
                .unwrap()
                .split(", ")
                .filter_map(|element| {
                    let mut words = element.splitn(2, ' ');
                    let n = match words.next()? {
                        "no" => None,
                        n => n.parse::<usize>().ok(),
                    }?;
                    let inner = words.next()?.rsplitn(2, ' ').skip(1).next()?.to_string();
                    (inner, n).into()
                })
                .collect::<HashMap<String, usize>>();
            (color, rules)
        })
        .collect()
}

fn can_reach_gold_bag(bags: &BagContainers, color: &str) -> bool {
    bags[color]
        .iter()
        .any(|(color, _)| color == "shiny gold" || can_reach_gold_bag(bags, color))
}

fn bag_count(bags: &BagContainers, color: &str) -> usize {
    1_usize
        + bags[color]
            .iter()
            .map(|(color, count)| count * bag_count(bags, color))
            .sum::<usize>()
}
