use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_str!("input.txt");
    let passports = file
        .split("\n\n")
        .filter_map(parse_passport)
        .collect::<Vec<_>>();

    let part_two = passports
        .iter()
        .filter(|passport| is_valid_passport(passport))
        .count();
    println!("Part one: {}", passports.len());
    println!("Part two: {}", part_two);
    Ok(())
}

fn parse_passport(s: &str) -> Option<HashMap<&str, &str>> {
    let passport = s
        .split_whitespace()
        .flat_map(|p| p.split(':'))
        .tuples()
        .collect::<HashMap<_, _>>();

    let invalid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .any(|k| !passport.contains_key(k));

    if invalid {
        return None;
    }
    Some(passport)
}

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "hcl" => {
            v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        "hgt" => {
            let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
            match &v[(v.len() - 2)..] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false,
            }
        }
        _ => unreachable!(),
    })
}
