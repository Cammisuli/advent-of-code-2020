fn main() {
    println!("Hello, world!");

    let seats = inputs(include_str!("inputs.txt"));

    println!("{}", part_one(seats.clone()))
}

fn part_one(seats: Vec<&str>) -> i32 {
    0
}

fn inputs(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_count_seats() {
        let total = part_one(inputs(
            r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#,
        ));

        assert_eq!(total, 37)
    }
}
