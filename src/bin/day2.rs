use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day2").unwrap();
    let input = input.as_str();
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input.lines()
        .filter(|line| is_safe(&parse_levels(line)))
        .count()
}

fn parse_levels(line: &str) -> Vec<usize> {
    let parts = line.split(" ");
    let levels: Vec<usize> = parts.map(|x| x.parse::<usize>().unwrap()).collect();
    levels
}

fn is_safe(levels: &Vec<usize>) -> bool {
    let increasing = levels[0] < levels[1];
    let mut safe = true;
    for i in 0..levels.len() - 1 {
        if increasing {
            if levels[i] >= levels[i + 1] {
                safe = false;
            } else if levels[i + 1] - levels[i] > 3 {
                safe = false;
            }
        } else {
            if levels[i] <= levels[i + 1] {
                safe = false;
            } else if levels[i] - levels[i + 1] > 3 {
                safe = false;
            }
        }
    }
    safe
}

fn part_2(input: &str) -> usize {
    input.lines()
        .filter(|line| {
            let levels = parse_levels(line);

            let safe = is_safe(&levels);
            if safe {
                return safe;
            } else {
                let mut levels_dampened = vec![0; levels.len() - 1];
                for i in 0..levels.len() {
                    levels_dampened[..i].copy_from_slice(&levels[..i]);
                    levels_dampened[i..].copy_from_slice(&levels[i+1..]);

                    if is_safe(&levels_dampened) {
                        return true;
                    }
                }
            }
            safe
        })
        .count()
}

const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 2);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 4);
}
