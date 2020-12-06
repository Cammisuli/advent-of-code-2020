fn main() {
    let input = inputs().unwrap();

    let highest_seat = part_one(input.clone());
    println!("{:?}", highest_seat);

    let my_seat = part_two(input);
    println!("{:?}", my_seat);
}

fn inputs() -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let inputs = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.replace(&['F', 'L'][..], "0")
                .replace(&['B', 'R'][..], "1")
        })
        .map(|binary| usize::from_str_radix(&binary, 2).unwrap())
        .collect();
    Ok(inputs)
}

fn part_one(seats: Vec<usize>) -> usize {
    *seats.iter().max().unwrap()
}

fn part_two(seats: Vec<usize>) -> usize {
    let mut assigned = [false; 1024];
    seats.iter().for_each(|&s| assigned[s] = true);
    (1..=1022)
        .find(|&i| assigned[i - 1] && !assigned[i])
        .unwrap()
}
