use std::io::Error;
use common::read_lines;
use std::collections::HashMap;

type PassportData = HashMap<String, String>;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

const VALIDATORS: [fn(&str) -> bool; 7] = [
    |v| {
        v.parse().map_or(false, |n:usize| {
            n >= 1920 && n <= 2002
        })
    },
    |v| {
        v.parse().map_or(false, |n:usize| {
            n >= 2010 && n <= 2020
        })
    },
    |v| {
        v.parse().map_or(false, |n:usize| {
            n >= 2020 && n <= 2030
        })
    },
    |v| {
        if v.ends_with("cm") {
            v[..v.len()-2].parse().map_or(false, |n:usize| {
                n >= 150 && n <= 193
            })
        }
        else if v.ends_with("in") {
            v[..v.len()-2].parse().map_or(false, |n:usize| {
                n >= 59 && n <= 76
            })
        }
        else {
            false
        }
    },
    |v| {
        v.starts_with('#') && v.len() == 7
    },
    |v| {
        VALID_EYE_COLORS.iter().any(|&ec| v == ec)
    },
    |v| {
        v.len() == 9
    }
];

fn main() {
    let data = load_passports().expect("failed to load passport data");
    let valid = data.iter()
        .filter(|data| check_valid_part1(data))
        .count();

    println!("Part 1 - {:?} passports are valid", valid);

    let valid_part2 = data.iter()
        .filter(|data| check_valid_part2(data))
        .count();

    println!("Part 2 - {} passwords are valid", valid_part2);
}

fn check_valid_part1(passport: &PassportData) -> bool {
    for key in REQUIRED_KEYS.iter() {
        if !passport.contains_key(*key) {
            return false;
        }
    }
    true
}

fn check_valid_part2(passport: &PassportData) -> bool {
    for (key_idx, key) in REQUIRED_KEYS.iter().enumerate() {
        if !passport.contains_key(*key) || !VALIDATORS[key_idx](passport.get(*key).unwrap()) {
            return false;
        }
    }
    true
}

fn load_passports() -> Result<Vec<PassportData>, Error> {
    let mut passports: Vec<PassportData> = Vec::new();
    let lines = read_lines("./data/day4.txt")?;
    let mut data: PassportData = PassportData::new();
    for line in lines {
        if line.is_empty() {
            passports.push(data);
            data = PassportData::new();
        }

        let mut iter_w = line.split_whitespace();
        while let Some(pair) = iter_w.next() {
            let mut iter_c = pair.split(':');
            let key = iter_c.next().expect("missing key");
            let value = iter_c.next().expect("missing value");
            data.insert(String::from(key), String::from(value));
        }
    }
    if !data.is_empty() {
        passports.push(data);
    }
    Ok(passports)
}