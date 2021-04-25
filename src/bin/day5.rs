use std::io::Error;
use common::read_lines;

fn main() -> Result<(), Error> {

    let codes = read_lines("./data/day5.txt")?;

    let max_id = codes.iter()
        .map(|code| calc_seat(code.as_str()))
        .max_by(|a: &Seat, b: &Seat| a.id.cmp(&b.id))
        .unwrap();

    println!("Part 1 - max ID is {}", max_id.id);

    let mut ids: Vec<i32> = codes.iter()
        .map(|code| calc_seat(code).id)
        .collect();

    ids.sort_unstable();

    for (i, id) in ids.iter().enumerate() {
        if ids[i + 1] == id + 2 {
            println!("Part 2 - my seat ID is {}", id + 1);
            break;
        }
    }


    Ok(())
}

#[derive(PartialEq, Debug)]
struct Seat {
    row: i32,
    col: i32,
    id: i32,
}

fn midpoint(range: (i32, i32)) -> i32 {
    range.0 + (range.1 + 1 - range.0) / 2
}

fn calc_seat(code: &str) -> Seat {
    let mut row_range = (0, 127);
    let mut col_range = (0, 7);

    for c in code.chars() {
        match c {
            'F' => {
                row_range.1 = midpoint(row_range);
            },
            'B' => {
                row_range.0 = midpoint(row_range);
            },
            'L' => {
                col_range.1 = midpoint(col_range);
            },
            'R' => {
                col_range.0 = midpoint(col_range);
            },
            _ => ()
        }
    }

    Seat {
        row: row_range.0,
        col: col_range.0,
        id: row_range.0 * 8 + col_range.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_seat() {
        assert_eq!(calc_seat("FBFBBFFRLR"), Seat { row: 44, col: 5, id: 357});
    }
}

