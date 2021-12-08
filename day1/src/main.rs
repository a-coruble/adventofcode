use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<i64> {
    let filename = "./input.txt";
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| String::from(line.unwrap()).parse::<i64>().unwrap())
        .collect()
}

fn find_increases(values: Vec<i64>) -> i32 {
    let mut cur_value = values[0];
    let mut number_of_increments = 0;
    values.into_iter().for_each(|depth| {
        if depth > cur_value {
            number_of_increments += 1;
        }
        cur_value = depth
    });
    number_of_increments
}

fn group_and_sum_inputs_by_three(values: &Vec<i64>) -> Vec<i64> {
    values
        .into_iter()
        .enumerate()
        .take_while(|(index, _)| index + 2 < values.len())
        .map(|(index, value)| value + values[index + 1] + values[index + 2])
        .collect()
}

fn solve_step_1() {
    let depths = parse_input();
    let answer = find_increases(depths);
    println!("Solution to Step 1 : {}", answer)
}

fn solve_step_2() {
    let depths = parse_input();
    let sliding_windows = group_and_sum_inputs_by_three(&depths);
    let result = find_increases(sliding_windows);
    println!("Solution to Step 2 : {}", result);
}

fn main() {
    solve_step_1();
    solve_step_2()
}
