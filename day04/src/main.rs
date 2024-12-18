use std::env;
use std::fs;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let graphemes: Vec<&str> = contents.graphemes(true).filter(|&g| g != "\n").collect::<Vec<&str>>();
    let width: usize = contents.clone().graphemes(true).into_iter().position(|g| g == "\n").unwrap();
    let length = graphemes.clone().iter().count();
    let indexes_vertical = transpose(vertical_starters, vertical_calculate, width, length);
    let contents_vertical = move_indexes(graphemes.clone(), indexes_vertical);
    let indexes_left_diagonal = transpose(diagonal_left_starters, diagonal_left_calculate, width, length);
    let contents_left_diagonal = move_indexes(graphemes.clone(), indexes_left_diagonal.clone());
    let indexes_right_diagonal = transpose(diagonal_right_starters, diagonal_right_calculate, width, length);
    let contents_right_diagonal = move_indexes(graphemes.clone(), indexes_right_diagonal.clone());
    let horizontal_count = xmas_counter(contents);
    let vertical_count = xmas_counter(contents_vertical);
    let diagonal_left_count = xmas_counter(contents_left_diagonal.clone());
    let diagonal_right_count = xmas_counter(contents_right_diagonal.clone());
    let result_part1 = vertical_count + horizontal_count + diagonal_left_count + diagonal_right_count;

    // result of searching for "XMAS" left, right, up, down, diagonal left, diagonal right and overlapping
    println!("Part1 = {result_part1} h {horizontal_count} v {vertical_count} d {diagonal_left_count} {diagonal_right_count}");

    let mut left_x_result = mas_x_finder(contents_left_diagonal.clone(), indexes_left_diagonal.clone());
    let mut right_x_result = mas_x_finder(contents_right_diagonal.clone(), indexes_right_diagonal.clone());
    left_x_result.sort();
    right_x_result.sort();
    let result_part2 = left_x_result.iter().filter(|&p| right_x_result.contains(p)).count();

    // result of "MAS" in 2 diagonal directions sharing the "A", so it forms an X pattern 
    println!("Part2 = {}", result_part2)
}

/* find_iter results are non-overlapping, must search forwards and backwards separately to handle XMASAMX and SAMXMAS cases */
fn xmas_counter(contents: String) -> i32 {
    Regex::new(r"XMAS").unwrap().find_iter(contents.as_str()).count() as i32
        + Regex::new(r"SAMX").unwrap().find_iter(contents.as_str()).count() as i32
}

fn vertical_starters(width: usize) -> Vec<usize> {
    (0..width).collect()
}

fn diagonal_left_starters(width: usize) -> Vec<usize> {
    (0..width).chain((2..=width).map(|n| n * width - 1)).collect()
}

fn diagonal_right_starters(width: usize) -> Vec<usize> {
    (0..width).chain((1..width).map(|n| n * width)).collect()
}

fn vertical_calculate(starter: usize, width: usize) -> usize {
    starter + width
}

fn diagonal_left_calculate(starter: usize, width: usize) -> usize {
    starter + width - 1
}

fn diagonal_right_calculate(starter: usize, width: usize) -> usize {
    starter + width + 1
}

fn transpose(f_starters: fn(usize) -> Vec<usize>, f_calculate: fn(usize, usize) -> usize, width: usize, length: usize) -> Vec<Vec<usize>> {
    let starters = f_starters(width);
    let mut result: Vec<Vec<usize>> = Vec::new();
    for starter in starters.clone() {
        let mut row: Vec<usize> = Vec::new();
        row.push(starter);
        let mut value = starter;
        for _ in 0..width {
            value = f_calculate(value, width);
            if starters.clone().iter().any(|&v| v == value) || value >= length {
                break;
            } else {
                row.push(value);
            }
        }
        result.push(row);
    }
    result
}

fn move_indexes(contents: Vec<&str>, indexes: Vec<Vec<usize>>) -> String {
    let mut result: Vec<&str> = Vec::new();
    for index_vec in indexes {
        for index in index_vec {
            result.push(contents[index]);
        }
        result.push("\n");
    }
    result.into_iter().collect()
}

// find pre-diagonalized index position of "A", add to a left and right lists, compare
fn mas_x_finder(contents: String, indexes: Vec<Vec<usize>>) -> Vec<usize> {
    let mut result_index: usize = 0;
    let mut result:Vec<usize> = Vec::new();
    for line in contents.lines() {
        let matches: Vec<_> = line.match_indices("MAS").chain(line.match_indices("SAM")).collect();
        if !matches.is_empty() {
            for item in matches {
                result.push(indexes.clone()[result_index][item.0 + 1]);

            }
        }
        result_index = result_index + 1;
    }
    result
}