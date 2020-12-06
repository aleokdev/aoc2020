extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle6.txt"));

    // Puzzle 1
    let mut answers = HashSet::<char>::new();
    let mut counts = 0;
    for line in cursor.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            counts += answers.len();
            answers = HashSet::new();
            continue;
        }

        for ch in line.chars() {
            answers.insert(ch);
        }
    }

    counts += answers.len();

    println!("{}", counts);

    let cursor = Cursor::new(include_str!("inputs/puzzle6.txt"));

    // Puzzle 2
    let mut answers = ('a'..='z').collect::<HashSet<_>>();

    let mut counts = 0;
    for (i, line) in cursor.lines().enumerate() {
        let line = line.unwrap();
        let mut person_answers = HashSet::<char>::new();
        if line.is_empty() {
            counts += answers.len();
            answers = ('a'..='z').collect::<HashSet<_>>();
            continue;
        }

        for ch in line.chars() {
            person_answers.insert(ch);
        }
        answers = answers.intersection(&person_answers).cloned().collect();
    }

    counts += answers.len();

    println!("{}", counts);
}