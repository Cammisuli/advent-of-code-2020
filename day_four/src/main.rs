extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pairs;
use pest::Parser;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

#[derive(Parser)]
#[grammar = "day_four.pest"]
pub struct InputParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_to_string("../input/day_four.txt")?;
    let mut input = InputParser::parse(Rule::file, &file).unwrap();

    let valid = part_one(&mut input.clone());
    println!("{:?}", valid);
    let valid = part_two(&mut input.clone());
    println!("{:?}", valid);
    Ok(())
}

fn part_one(inputs: &mut Pairs<Rule>) -> i32 {
    let mut valid_passports = 0;
    for passport in inputs {
        match passport.as_rule() {
            Rule::passport => {
                if passport
                    .into_inner()
                    .map(|record| record.as_str().split(":").next())
                    .filter(|&str| str != Some("cid"))
                    .count()
                    == 7
                {
                    valid_passports += 1;
                }
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    valid_passports
}

fn part_two(inputs: &mut Pairs<Rule>) -> i32 {
    let mut valid_passports = 0;

    for passport in inputs {
        match passport.as_rule() {
            Rule::passport => {
                if passport
                    .into_inner()
                    .map(|record| record.as_str())
                    .filter(|field| {
                        let mut kv = field.split(":");
                        let k = kv.next().unwrap();
                        let v = kv.next().unwrap();
                        match k {
                            "byr" => is_number_in_range(v, 1920..=2002),
                            "iyr" => is_number_in_range(v, 2010..=2020),
                            "eyr" => is_number_in_range(v, 2020..=2030),
                            "hgt" => {
                                is_number_in_range_suffixed(v, 150..=193, "cm")
                                    || is_number_in_range_suffixed(v, 59..=76, "in")
                            }
                            "hcl" => {
                                v.len() == 7
                                    && v.strip_prefix("#")
                                        .map_or(false, |r| r.chars().all(|c| c.is_ascii_hexdigit()))
                            }
                            "ecl" => {
                                matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                            }
                            "pid" => v.len() == 9 && v.chars().all(|c| c.is_numeric()),
                            _ => false,
                        }
                    })
                    .count()
                    == 7
                {
                    valid_passports += 1;
                }
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }
    valid_passports
}

fn is_number_in_range(v: &str, range: RangeInclusive<u16>) -> bool {
    v.parse().map_or(false, |n: u16| range.contains(&n))
}

fn is_number_in_range_suffixed(v: &str, range: RangeInclusive<u16>, suffix: &str) -> bool {
    v.strip_suffix(suffix)
        .map_or(false, |rest| is_number_in_range(rest, range))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_records() {
        let parsed = InputParser::parse(Rule::record, "hcl:#fffffd");
        println!("{:?}", parsed);
        assert!(parsed.is_ok())
    }

    #[test]
    fn parses_passports() {
        let parsed =
            InputParser::parse(Rule::passport, "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd");
        println!("{:?}", parsed);
        assert!(parsed.is_ok());
        let result = parsed.unwrap();
        assert_eq!(result.count(), 1);

        let parsed_newline = InputParser::parse(
            Rule::passport,
            r#"hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm"#,
        );
        assert!(parsed_newline.is_ok());
        assert_eq!(parsed_newline.unwrap().count(), 1);
    }

    #[test]
    fn parses_file() {
        let parsed = InputParser::parse(
            Rule::file,
            r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#,
        );
        println!("{:?}", parsed);
        assert!(parsed.is_ok());
        // EOI has it's own Pair
        assert_eq!(parsed.unwrap().count(), 5);
    }
}
