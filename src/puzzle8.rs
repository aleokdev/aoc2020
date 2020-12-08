extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashSet;

struct Instruction {
    op: String,
    offset: i32
}

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle8.txt"));
    let re = Regex::new(r"([^ ]+) (.+)").unwrap();
    let mut instructions = Vec::new();
    for line in cursor.lines() {
        let line = line.unwrap();
        let captures = re.captures(line.as_str()).unwrap();
        instructions.push(
            Instruction {
                op: String::from(captures.get(1).unwrap().as_str()),
                offset: i32::from_str(captures.get(2).unwrap().as_str()).unwrap()
            }
        );
    }

    // no part 1 for you

    let mut pc: i32 = 0;
    let mut acc = 0;

    fn try_jmp(instructions: &Vec<Instruction>, mut pc: i32, mut acc: i32) -> Option<i32> {
        let mut exec_ins: HashSet<i32> = HashSet::new();

        while let Some(ins) = instructions.get(pc as usize) {
            if !exec_ins.insert(pc) {
                return None;
            }

            match ins.op.as_str() {
                "acc" => {
                    acc += ins.offset;
                },
                "jmp" => {
                    pc += ins.offset;
                    continue;
                },
                "nop" => (),
                _ => unreachable!()
            }
            pc += 1;
        }

        Some(acc)
    }

    let mut exec_ins: HashSet<i32> = HashSet::new();

    while let Some(ins) = instructions.get(pc as usize) {
        if !exec_ins.insert(pc) {
            panic!();
        }

        match ins.op.as_str() {
            "acc" => {
                acc += ins.offset;
            },
            "jmp" => {
                // Try to change the jmp into a nop
                if let Some(x) = try_jmp(&instructions, pc + 1, acc) {
                    acc = x;
                    break;
                } else {
                    pc += ins.offset;
                    continue;
                }
            },
            "nop" => {
                // Try to change the nop into a jmp
                if let Some(x) = try_jmp(&instructions, pc + ins.offset, acc) {
                    acc = x;
                    break;
                }
            },
            _ => unreachable!()
        }
        pc += 1;
    }

    println!("Terminated! {}", acc);
}