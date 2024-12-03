use std::env;
use std::fs;
use std::ops::Mul;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let mut result_part1: i32 = 0;
    // match "mul(nnn,nnn)"
    let re1: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for capture in re1.captures_iter(contents.as_str()) {
        let left_op = &capture[1].to_ascii_lowercase().parse::<i32>().unwrap();
        let right_op = &capture[2].to_ascii_lowercase().parse::<i32>().unwrap();
        result_part1 = result_part1 + (left_op.mul(right_op));
    }
    println!("Part1 = {result_part1}");

    let mut result_part2: i32 = 0;
    let mut enabled: bool = true;
    // match "mul(nnn,nnn)" or "do()" or "don't()"
    let re2: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don\'t\(\)").unwrap();
    for capture in re2.captures_iter(contents.as_str()) {
        if capture[0].starts_with("do()") {
            enabled = true;
        } else if capture[0].starts_with("don't()") {
            enabled = false;
        } else if enabled {
            let left_op = &capture[1].to_ascii_lowercase().parse::<i32>().unwrap();
            let right_op = &capture[2].to_ascii_lowercase().parse::<i32>().unwrap();
            result_part2 = result_part2 + (left_op.mul(right_op));
        }
    }
    println!("Part2 = {result_part2}");
}
