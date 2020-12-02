use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct InputLine {
    min: u32,
    max: u32,
    character: char,
    value: String,
}

fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = inputs()?;

    let mut passwords = Vec::new();
    for input_line in &input {
        let value = input_line.value.chars();

        let character_counts = value.filter(|&x| x == input_line.character).count() as u32;
        if character_counts >= input_line.min && character_counts <= input_line.max {
            passwords.push(&input_line.value);
        }
    }
    println!("{:?}", passwords.len());
    Ok(())
}

fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let input = inputs()?;

    let mut passwords = Vec::new();
    for input_line in &input {
        let value_chars = input_line.value.chars().collect::<Vec<_>>();
        let min = (input_line.min - 1) as usize;
        let max = (input_line.max - 1) as usize;

        let min_max = (
            value_chars.get(min).filter(|&&m| m == input_line.character),
            value_chars.get(max).filter(|&&m| m == input_line.character),
        );

        match min_max {
            (Some(&min), None) => passwords.push(&input_line.value),
            (None, Some(&max)) => passwords.push(&input_line.value),
            _ => {}
        }
    }

    println!("{:?}", passwords.len());

    Ok(())
}

fn inputs() -> Result<Vec<InputLine>, Box<dyn Error>> {
    let file = read_to_string("input/day_two.txt")?;
    let input = file
        .lines()
        .map(|x| {
            let (a, value) = match x.split(":").collect::<Vec<_>>()[..] {
                [a, b] => (a, b),
                _ => unreachable!(),
            };

            let (min_max, character) = match a.split_whitespace().collect::<Vec<_>>()[..] {
                [a, b] => (a, b),
                _ => unreachable!(),
            };

            let (min, max) = match min_max.split('-').collect::<Vec<_>>()[..] {
                [min, max] => (min, max),
                _ => unreachable!(),
            };

            InputLine {
                min: min.parse().unwrap_or_default(),
                max: max.parse().unwrap_or_default(),
                character: character.chars().nth(0).unwrap_or('a'),
                value: value.trim().to_owned(),
            }
        })
        .collect::<Vec<_>>();

    Ok(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        part_one().unwrap();
        part_two().unwrap();
    }
}
