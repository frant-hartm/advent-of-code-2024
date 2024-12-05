use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day5").unwrap();
    let input = input.as_str();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let (pairs, updates) = parse_pairs_and_updates(input);

    updates
        .into_iter()
        .filter(|update| is_ordered(update, &pairs))
        .map(|update| get_middle(&update))
        .sum()
}

fn parse_pairs_and_updates(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (pairs, updates) = input.split_once("\n\n").unwrap();

    let pairs: Vec<(usize, usize)> = pairs
        .lines()
        .map(|line| {
            let (s1, s2) = line.split_once("|").unwrap();
            let a = s1.parse::<usize>().unwrap();
            let b = s2.parse::<usize>().unwrap();

            (a, b)
        })
        .collect();

    let updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|part| part.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    (pairs, updates)
}

fn is_ordered(update: &Vec<usize>, pairs: &Vec<(usize, usize)>) -> bool {
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            // if pairs contains a pair in reverse order the update is not ordered
            let pair = (update[j], update[i]);
            if pairs.contains(&pair) {
                return false;
            }
        }
    }
    // println!("Valid: {:#?}", update);
    true
}

fn get_middle(list: &Vec<usize>) -> usize {
    if list.len() % 2 == 0 {
        panic!("Should receive odd number of elements, vec: {:#?}", list);
    }

    list[list.len() / 2]
}

fn part_2(input: &str) -> usize {
    let (pairs, mut updates) = parse_pairs_and_updates(input);

    updates
        .iter_mut()
        .filter(|update| !is_ordered(update, &pairs))
        .map(|update| {
            sort(update, &pairs);
            get_middle(&update)
        })
        .sum()
}

fn sort(update: &mut Vec<usize>, pairs: &Vec<(usize, usize)>) {
    update.sort_by(|a, b| {
        if pairs.contains(&(*a, *b)) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    })
}

const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[test]
fn test_part_1() {
    assert_eq!(part_1(EXAMPLE), 143);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(EXAMPLE), 123);
}
