use regex::{Match, Regex};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day3").unwrap();
    let input = input.as_str();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| multiply_captures(c.get(1), c.get(2)))
        .sum()
}

fn multiply_captures(c1: Option<Match>, c2: Option<Match>) -> usize {
    if let (Some(o1), Some(o2)) = (c1, c2) {
        let first: usize = o1.as_str().parse().unwrap();
        let second: usize = o2.as_str().parse().unwrap();
        first * second
    } else {
        0
    }
}

fn part_2(input: &str) -> usize {
    let re = Regex::new(r"(mul|do|don't)\((\d*),?(\d*)\)").unwrap();

    let mut active = true;
    let mut total = 0;
    for c in re.captures_iter(input) {
        // println!("Capture: {:#?}", c);
        let instruction = c.get(1).unwrap().as_str();

        match instruction {
            "mul" => {
                if active {
                    total += multiply_captures(c.get(2), c.get(3))
                }
            }
            "do" => {
                active = true;
            }
            "don't" => {
                active = false;
            }
            _ => panic!("Unknown instruction {}", instruction),
        }
    }
    total
}

const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 161);
}

const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE2), 48);
}
