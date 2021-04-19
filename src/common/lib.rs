use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("couldn't read line"))
        .collect()
}

pub fn read_ints(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("couldn't read line"))
        .map(|l| l.parse().expect("not a integer"))
        .collect()
}
