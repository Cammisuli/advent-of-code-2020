fn main() -> Result<(), String> {
    let (start_time, buses) = inputs().ok_or("No inputs")?;

    println!("{}", part_one(start_time, &buses));
    println!("{}", part_two(&buses));

    Ok(())
}

fn part_one(start_time: i64, buses: &[(i64, i64)]) -> i64 {
    for i in start_time.. {
        if let Some((_, bus)) = buses.iter().find(|&(_, bus)| i % bus == 0) {
            return bus * (i - start_time);
        }
    }
    00
}

fn part_two(buses: &[(i64, i64)]) -> i64 {
    let mods = buses.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    let res = buses.iter().map(|(i, b)| b - i).collect::<Vec<_>>();
    chinese_remainder(&res, &mods).unwrap()
}

fn inputs() -> Option<(i64, Vec<(i64, i64)>)> {
    let mut split = include_str!("inputs.txt").split('\n');
    let start_time = split.next()?.parse().ok()?;
    let buses: Vec<(i64, i64)> = split.next().map(|buses| {
        buses
            .split(',')
            .enumerate()
            .filter_map(|(i, s)| s.parse().ok().map(|v: i64| (i as i64, v)))
            .collect::<Vec<_>>()
    })?;

    Some((start_time, buses))
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
