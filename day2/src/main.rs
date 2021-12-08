use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Instruction {
    fn new(raw_instruction: String) -> Option<Instruction> {
        let splitted_instruction: Vec<&str> = raw_instruction.split(" ").collect();
        let value = String::from(splitted_instruction[1])
            .parse::<u32>()
            .unwrap();
        match splitted_instruction[0] {
            "forward" => Some(Instruction::Forward(value)),
            "up" => Some(Instruction::Up(value)),
            "down" => Some(Instruction::Down(value)),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Instruction::Forward(value) => write!(f, "Direction : Forward / Value : {}", value),
            Instruction::Down(value) => write!(f, "Direction : Forward / Value : {}", value),
            Instruction::Up(value) => write!(f, "Direction : Forward / Value : {}", value),
        }
    }
}

fn parse_input() -> Vec<String> {
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| String::from(line.unwrap()))
        .collect()
}

fn solve_step_1() {
    let mut depth = 0;
    let mut horizontal_position = 0;
    parse_input()
        .into_iter()
        .map(|raw_instruction| Instruction::new(raw_instruction))
        .for_each(|instruction| match instruction.unwrap() {
            Instruction::Forward(value) => horizontal_position += value,
            Instruction::Down(value) => depth += value,
            Instruction::Up(value) => depth -= value,
        });
    let result = depth * horizontal_position;
    println!("Solution to Step 1 :");
    println!("    - Depth : {}", depth);
    println!("    - Horizontal position : {}", horizontal_position);
    println!("");
    println!(" Total {}", result);
}

fn solve_step_2() {
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;
    parse_input()
        .into_iter()
        .map(|raw_instruction| Instruction::new(raw_instruction))
        .for_each(|instruction| match instruction.unwrap() {
            Instruction::Forward(value) => {
                horizontal_position += value;
                depth += value * aim;
            }
            Instruction::Down(value) => aim += value,
            Instruction::Up(value) => aim -= value,
        });
    let result = depth * horizontal_position;
    println!("Solution to Step 2 :");
    println!("    - Depth : {}", depth);
    println!("    - Horizontal position : {}", horizontal_position);
    println!("");
    println!(" Total {}", result);
}

fn main() {
    solve_step_1();
    solve_step_2()
}
