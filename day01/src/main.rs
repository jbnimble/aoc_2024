use std::env;
use std::fs;
use std::ops::Mul;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    
    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = contents.lines();

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in lines {
        let values: Vec<_> = line.split_whitespace().collect();
        let val1 = values[0].parse::<i32>().unwrap();
        let val2 = values[1].parse::<i32>().unwrap();

        vec1.push(val1);
        vec2.push(val2);
    }

    vec1.sort();
    vec2.sort();

    // part1
    let mut total_part1: i32 = 0;
    for n in 0..vec1.len() {
        let val1 = vec1[n];
        let val2 = vec2[n];
        let diff = (val2 - val1).abs();
        total_part1 = total_part1 + diff;
    }
    println!("Part1 = {total_part1}");

    // part2
    let mut total_part2: i32 = 0;
    for n in 0..vec1.len() {
        let val1 = vec1[n];
        let count = vec2.iter().filter(|x| val1.eq(x)).count() as i32;
        total_part2 = total_part2 + (val1.mul(count));
    }
    println!("Part2 = {total_part2}");
}
