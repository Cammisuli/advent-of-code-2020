fn main() {
    let part_one = part_one().unwrap();
    let part_two = part_two().unwrap();

    println!("part one: {} \npart two: {}", part_one, part_two)
}

fn part_one() -> Result<u32, Box<dyn std::error::Error>> {
    let input = inputs()?;
    let trees = count_trees(&input, 3, 1);
    Ok(trees)
}

fn part_two() -> Result<u32, Box<dyn std::error::Error>> {
    let input = inputs()?;
    let trees = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| count_trees(&input, *right, *down))
        .product::<u32>();
    Ok(trees)
}

fn count_trees(slopes: &Vec<String>, right: usize, down: usize) -> u32 {
    let (_, trees) = slopes
        .iter()
        .step_by(down)
        .fold((0, 0), |(pos, trees), slope| {
            match slope.chars().collect::<Vec<_>>().get(pos % slope.len()) {
                Some(&char) if char == '#' => (pos + right, trees + 1),
                _ => (pos + right, trees),
            }
        });

    trees
}

fn inputs() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = include_str!("input.txt");
    Ok(file.lines().map(|s| s.to_owned()).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work() {
        assert_eq!(part_one().unwrap(), 225);
        assert_eq!(part_two().unwrap(), 1115775000)
    }

    #[test]
    fn it_counts_trees() {
        assert_eq!(count_trees(&get_trees(), 3, 1), 7);

        assert_eq!(
            [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
                .iter()
                .map(|(right, down)| count_trees(&get_trees(), *right, *down))
                .product::<u32>(),
            336
        )
    }

    fn get_trees() -> Vec<String> {
        r#"
..##.........##.........##.........##.........##.........##.......
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"#
            .trim()
            .lines()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>()
    }
}
