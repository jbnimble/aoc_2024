use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let lines = contents.lines();

    let mut safe_reports: i32 = 0;
    let mut damp_reports: i32 = 0;

    for line in lines {
        let values: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let value_len = values.len();
        
        if is_safe_report(values.clone()) {
            safe_reports = safe_reports + 1;
        } else {
            for n in 0..value_len {
                let mut working = values.clone();
                working.remove(n);
                if is_safe_report(working) {
                    damp_reports = damp_reports + 1;
                    break;
                }
            }
        }
    }
    println!("Part1 = {safe_reports}");
    damp_reports = damp_reports + safe_reports;
    println!("Part2 = {damp_reports}");
}

fn is_safe_report(values: Vec<i32>) -> bool {
    let sorted_asc: bool = values.iter().is_sorted_by(|a, b| a < b);
    let sorted_dec: bool = values.iter().is_sorted_by(|a, b| a > b);
    let sorted_adj = values.iter()
        .is_sorted_by(|a, b| a.abs_diff(**b) >= 1 && a.abs_diff(**b) <= 3);
    (sorted_asc && sorted_adj) || (sorted_dec && sorted_adj)
}
