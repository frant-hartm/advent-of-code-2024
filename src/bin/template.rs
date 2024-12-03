use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/dayX").unwrap();
    let input = input.as_str();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input :&str) -> usize {
    0
}

fn part_2(input :&str) -> usize {
    0
}

const EXAMPLE: &str = "";

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 0);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 0);
}
