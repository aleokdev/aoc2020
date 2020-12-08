extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashSet;

#[macro_use] mod instructions;
use instructions::*;

fn main() {
    let mut instructions = read_instructions!("inputs/puzzle8.txt");

    // no part 1 for you

    let mut pc: i32 = 0;
    let mut acc = 0;

    fn try_jmp(instructions: &Vec<Instruction>, mut pc: i32, mut acc: i32) -> Option<i32> {
        let mut exec_ins: HashSet<i32> = HashSet::new();

        while let Some(ins) = instructions.get(pc as usize) {
            if !exec_ins.insert(pc) {
                return None;
            }

            match ins {
                Instruction::Acc(offset) => {
                    acc += offset;
                },
                Instruction::Jmp(offset) => {
                    pc += offset;
                    continue;
                },
                Instruction::Nop(_) => (),
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

        match ins {
            Instruction::Acc(offset) => {
                acc += offset;
            },
            Instruction::Jmp(offset) => {
                // Try to change the jmp into a nop
                if let Some(x) = try_jmp(&instructions, pc + 1, acc) {
                    acc = x;
                    break;
                } else {
                    pc += offset;
                    continue;
                }
            },
            Instruction::Nop(offset) => {
                // Try to change the nop into a jmp
                if let Some(x) = try_jmp(&instructions, pc + offset, acc) {
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