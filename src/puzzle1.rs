use std::io::{Cursor, BufRead};
use std::str::FromStr;

fn main() {
    let mut inputs = Vec::new();
    let cursor = Cursor::new(include_str!("inputs/puzzle1.txt"));
    for line in cursor.lines() {
        let line = line.unwrap();
        let line_n = i32::from_str(line.as_str()).unwrap();
        inputs.iter().cloned().for_each(|x| {
            inputs.iter().cloned().for_each(|y| {
                if x + y + line_n == 2020 {
                    println!("{}", x * y * line_n);
                }
        })
        });
        inputs.push(line_n);
    }
}
