use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day1").unwrap();
    let input = input.as_str();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input : &str) -> usize {
    let (l1, l2) = parse_lists(input);

    l1.iter().zip(l2.iter())
        .map(|(a, b)| {
            if a < b {
                b - a
            } else {
                a - b
            }
        })
        .sum()
}

fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut l1 = vec![];
    let mut l2 = vec![];

    input.lines().for_each(|line| {
        let mut split = line.split("   ");
        let first: usize = split.next().unwrap().parse().unwrap();
        let second: usize = split.next().unwrap().parse().unwrap();

        l1.push(first);
        l2.push(second);
    });

    l1.sort();
    l2.sort();

    (l1, l2)
}

fn part_2(input : &str) -> usize {
    let (l1, l2) = parse_lists(input);

    let mut total = 0;
    let mut start = 0;
    for i in 0..l1.len() {
        let number = l1[i];
        let mut count = 0;
        for j in start..l2.len() {
            if l2[j] < number {
                start += 1;
                continue;
            } else if l2[j] == number {
                count += 1;
            } else {
                break;
            }
        }

        // println!("Number {} times {}", n, c);
        total += number * count;
    }

    total
}

const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 11);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 31);
}
