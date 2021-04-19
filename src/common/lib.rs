use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> Result<Vec<String>, Error> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("couldn't read line"))
        .collect()
}

pub fn read_ints(filename: impl AsRef<Path>) -> Result<Vec<i32>, Error> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("couldn't read line"))
        .map(|l| l.parse().expect("not a integer"))
        .collect()
}
