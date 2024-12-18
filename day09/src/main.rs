use std::env;
use std::fs;
use std::iter;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let contents: String = fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("Failed to read file");

    let data1 = parse_data_part1(&contents);
    let result_part1 = calculate_part1(data1);
    println!("Part1 = {}", result_part1);

    let data2 = parse_data_part2(&contents);
    let result_part2 = calculate_part2(data2);
    println!("Part2 = {}", result_part2);
}

fn calculate_part1(mut data: Vec<isize>) -> isize {
    for index in 0..data.len() {
        if index >= data.len() {
            break;
        }
        let value = data[index];
        if value == -1 {
            let mut last_option = data.pop();
            while last_option.is_some() {
                if last_option.is_some_and(|f| f != -1) {
                    data[index] = last_option.unwrap();
                    last_option = Option::None;
                } else {
                    last_option = data.pop();
                }
            }
        }
    }
    let mut result: isize = 0;
    for index in 0..data.len() {
        result += index as isize * data[index];
    }
    result
}

fn calculate_part2(mut data: Vec<Block>) -> isize {
    loop {
        let empty_indexes = get_empty_indexes(&data);

        let last_unmoved_index_option = get_last_unmoved(&data);
        if last_unmoved_index_option.is_some() {
            let last_unmoved_index = last_unmoved_index_option.unwrap();
            data[last_unmoved_index].is_move_attempt = true;

            for empty_index in empty_indexes {
                if empty_index < last_unmoved_index && data[empty_index].length >= data[last_unmoved_index].length {
                    let moving_block = data.remove(last_unmoved_index);
                    if last_unmoved_index < data.len() && data[last_unmoved_index].value == -1 {
                        data[last_unmoved_index].length = data[last_unmoved_index].length + moving_block.length;
                    } else {
                        data.insert(last_unmoved_index, Block{value: -1 as isize, length: moving_block.length, is_move_attempt: false});
                    }
                    if data[empty_index].length > moving_block.length {
                        data[empty_index].length = data[empty_index].length - moving_block.length;
                    } else {
                        data.remove(empty_index);
                    }
                    data.insert(empty_index, moving_block);
                    break;
                }
            }
        } else {
            break;
        }
    }
    add_blocks(&data)
}

fn add_blocks(data: &Vec<Block>) -> isize {
    let mut result = 0;
    let mut index = 0;
    for block in data {
        if block.value == -1 {
            index += block.length;
        } else {
            for n in 0..block.length {
                result += block.value * index as isize;
                index += 1;
            }
        }
    }
    result
}

fn get_empty_indexes(data: &Vec<Block>) -> Vec<usize> {
    let mut empty_indexes = Vec::new();
    for index in 0..data.len() {
        if data[index].value == -1 {
            empty_indexes.push(index);
        }
    }
    empty_indexes
}

fn get_last_unmoved(data: &Vec<Block>) -> Option<usize> {
    let rev_unmoved_index = data.clone().iter().rev().position(|p| p.is_move_attempt == false && p.value != -1);
    if rev_unmoved_index.is_some() {
        Option::Some(data.len() - 1 - rev_unmoved_index.unwrap())
    } else {
        Option::None
    }
}

fn parse_data_part1(contents: &String) -> Vec<isize> {
    let graphemes: Vec<&str> = contents.graphemes(true).filter(|&g| g != "\n").collect::<Vec<&str>>();
    let mut file_index: usize = 0;
    let mut result: Vec<isize> = Vec::new();
    for index in 0..graphemes.len() {
        let value = graphemes[index].parse::<usize>().unwrap();
        if index % 2 == 0 {
            // file length
            iter::repeat("n").take(value).for_each(|_| result.push(file_index as isize));
            file_index += 1;
        } else {
            // free space length
            iter::repeat(".").take(value).for_each(|_| result.push(-1));
        }
    }
    result
}

fn parse_data_part2(contents: &String) -> Vec<Block> {
    let mut result: Vec<Block> = Vec::new();
    let graphemes: Vec<&str> = contents.graphemes(true).filter(|&g| g != "\n").collect::<Vec<&str>>();
    let mut file_index: usize = 0;
    for index in 0..graphemes.len() {
        let value = graphemes[index].parse::<usize>().unwrap();
        if index % 2 == 0 {
            // file length
            result.push(Block{value: file_index as isize, length: value, is_move_attempt: false});
            file_index += 1;
        } else {
            // free space length
            if value > 0 {
                result.push(Block{value: -1 as isize, length: value, is_move_attempt: false});
            }
        }
    }
    result
}

#[derive(Debug, Clone)]
struct Block {
    value: isize,
    length: usize,
    is_move_attempt: bool,
}
