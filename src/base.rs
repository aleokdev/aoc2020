extern crate regex;
use std::io::{Cursor, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle.txt"));
    for line in cursor.lines() {
        let line = line.unwrap();
    }
}