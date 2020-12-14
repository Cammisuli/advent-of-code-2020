use std::str::FromStr;

fn main() {
    let inputs = inputs();

    println!("part one: {:?}", part_one(&inputs));
    println!("part two: {:?}", part_two(&inputs));
}

fn part_one(directions: &[(char, isize)]) -> isize {
    let (mut x, mut y, mut r) = (0, 0, 90);
    for (d, n) in directions {
        match d {
            'N' => y += n,
            'S' => y -= n,
            'E' => x += n,
            'W' => x -= n,
            'L' => r -= n,
            'R' => r += n,
            'F' => match r % 360 {
                0 => y += n,
                90 => x += n,
                180 => y -= n,
                270 => x -= n,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    (x.abs() + y.abs()) as isize
}

fn rotate(x: isize, y: isize, d: isize) -> (isize, isize) {
    match d {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => unreachable!(),
    }
}

fn part_two(directions: &[(char, isize)]) -> isize {
    let (mut x, mut y) = (10, 1); // waypoint
    let (mut i, mut j) = (0, 0); // ship
    for &(d, n) in directions {
        match d {
            'N' => y += n,
            'S' => y -= n,
            'E' => x += n,
            'W' => x -= n,
            'L' => {
                let (a, b) = rotate(x, y, n);
                x = a;
                y = b;
            }
            'R' => {
                let (a, b) = rotate(x, y, 360 - n);
                x = a;
                y = b;
            }
            'F' => {
                i += x * n;
                j += y * n;
            }
            _ => unreachable!(),
        }
    }
    (i.abs() + j.abs()) as isize
}

fn inputs() -> Vec<(char, isize)> {
    include_str!("inputs.txt")
        .lines()
        .map(|s| {
            let (direction, units) = s.split_at(1);
            (char::from_str(direction).unwrap(), units.parse().unwrap())
        })
        .collect()
}
