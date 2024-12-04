use crate::Direction::{DiagonalDown, DiagonalUp, Down, Right};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day4").unwrap();
    let input = input.as_str();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    DiagonalUp,
    DiagonalDown,
}

impl Direction {
    fn next(&self, (row, col): (usize, usize), size: usize) -> Option<(usize, usize)> {
        let (row, col) = (row as isize, col as isize);
        let (row, col) = match self {
            Down => (row + 1, col),
            Right => (row, col + 1),
            DiagonalUp => (row - 1, col + 1),
            DiagonalDown => (row + 1, col + 1),
        };
        if 0 <= row && row <= size as isize - 1 && 0 <= col && col <= size as isize - 1 {
            Some((row as usize, col as usize))
        } else {
            None
        }
    }
}

fn part_1(input: &str) -> usize {
    let plan = parse_plan(input);
    let words = vec![['X', 'M', 'A', 'S'], ['S', 'A', 'M', 'X']];

    let mut total = 0;
    for i in 0..plan.len() {
        for j in 0..plan.len() {
            for word in words.iter() {
                for dir in [Right, Down, DiagonalUp, DiagonalDown] {
                    if matches(&plan, word, (i, j), dir) {
                        // println!("Found {:#?} at {:?}", word, (i, j));
                        total += 1;
                    }
                }
            }
        }
    }
    total
}

fn parse_plan(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            Vec::from(bytes)
        })
        .collect()
}

fn matches(plan: &Vec<Vec<u8>>, word: &[char], (i, j): (usize, usize), dir: Direction) -> bool {
    let mut coord = Some((i, j));
    for n in 0..word.len() {
        if let Some((row, col)) = coord {
            if word[n] != plan[row][col] as char {
                return false;
            }
            coord = dir.next((row, col), plan.len());
        } else {
            return false;
        }
    }

    true
}

fn part_2(input: &str) -> usize {
    let plan = parse_plan(input);
    let words = vec![['M', 'A', 'S'], ['S', 'A', 'M']];

    let mut total = 0;
    for i in 0..plan.len() - 2 {
        for j in 0..plan.len() - 2 {
            let mut down = false;
            let mut up = false;
            for word in words.iter() {
                down = down || matches(&plan, word, (i, j), DiagonalDown);
                up = up || matches(&plan, word, (i + 2, j), DiagonalUp);
            }
            if up && down {
                total += 1;
            }
        }
    }
    total
}

const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 18);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 9);
}
