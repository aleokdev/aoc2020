extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::VecDeque;

fn main() {
    const BUFFER_SIZE: usize = 25;
    let cursor = Cursor::new(include_str!("inputs/puzzle9.txt"));
    let mut data = Vec::new();
    for line in cursor.lines() {
        let line = line.unwrap();
        let int = usize::from_str(line.as_str()).unwrap();
        data.push(int);
    }
    let check_if_sum = |index: usize| -> bool {
        for i in (index-BUFFER_SIZE).max(0)..index {
            for j in (index-BUFFER_SIZE).max(0)..index {
                if i == j { continue; }
                if data[index] == data[i] + data[j] { return true; }
            }
        }
        false
    };
    let invalid_num: usize = {
        let mut result = 0;
        for i in BUFFER_SIZE..data.len() {
            if !check_if_sum(i) {
                println!("Invalid number: {}", data[i]);
                result = data[i];
                break;
            }
        }
        result
    };
    for i in 0..data.len() {
        for size in 1..data.len()-i {
            let mut sum: usize = 0;
            let mut smallest: usize = 9999999;
            let mut largest: usize = 0;
            for j in i..i+size {
                sum += data[j];
                if data[j] < smallest {
                    smallest = data[j];
                }
                if data[j] > largest {
                    largest = data[j];
                }
            }
            if sum == invalid_num {
                println!("{} found, {}..{}", sum, i, i+size-1);
                println!("Smallest + largest: {}", smallest + largest);
                return;
            }
        }
    }
    println!("Found no matches");
}