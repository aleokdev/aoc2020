use std::str::FromStr;
use regex::Regex;

pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([^ ]+) (.+)").unwrap();
        let captures = re.captures(s).unwrap();
        Ok(match captures.get(1).unwrap().as_str() {
            "acc" => {
                Self::Acc(i32::from_str(captures.get(2).unwrap().as_str()).unwrap())
            },
            "jmp" => {
                Self::Jmp(i32::from_str(captures.get(2).unwrap().as_str()).unwrap())
            },
            "nop" => {
                Self::Nop(i32::from_str(captures.get(2).unwrap().as_str()).unwrap())
            },
            _ => unimplemented!()
        })
    }
}

macro_rules! read_instructions {
    ($path: literal) => {{
        let cursor = Cursor::new(include_str!($path));
        let mut instructions = Vec::new();
        for line in cursor.lines() {
            let line = line.unwrap();
            instructions.push(
                $crate::instructions::Instruction::from_str(line.as_str()).unwrap()
            );
        }
        instructions
    }}
}