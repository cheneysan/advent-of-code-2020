#[macro_use] extern crate lazy_static;
use regex::Regex;
use common::read_lines;

fn main() {
    let lines = read_lines("./data/day2.txt");
    let pt1_valid_count = lines.iter()
        .map(parse_line)
        .filter(filter_policy_part1)
        .count();

    println!("Part 1 - there are {} valid passwords!", pt1_valid_count);

    let pt2_valid_count = lines.iter()
        .map(parse_line)
        .filter(filter_policy_part2)
        .count();

    println!("Part 2 - there are {} valid passwords!", pt2_valid_count);

}

fn parse_line(line: &String) -> (usize, usize, char, &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (.*)$").unwrap();
    }

    let caps = RE.captures(line).unwrap();
    let min: usize = caps.get(1).unwrap().as_str().parse().unwrap();
    let max: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let letter: char = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let password = caps.get(4).unwrap().as_str();
    (min, max, letter, password)
}

fn filter_policy_part1(policy_and_pass: &(usize, usize, char, &str)) -> bool {
    let (min, max, letter, pass) = policy_and_pass;
    let occs = pass.chars().filter(|c| c == letter).count();
    occs >= *min && occs <= *max
}

fn filter_policy_part2(policy_and_pass: &(usize, usize, char, &str)) -> bool {
    let (first, second, letter, pass) = policy_and_pass;
    let first_matches = pass.chars().nth(*first - 1).map_or(false, |c| c == *letter);
    let second_matches = pass.chars().nth(*second - 1).map_or(false, |c| c == *letter);
    first_matches ^ second_matches
}