use std::env;
use std::fs;
use std::ops::Add;
use std::ops::Mul;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let equations = parse_equations(contents);

    let mut result_part1: usize = 0;
    let mut result_part2: usize = 0;

    for mut equation in equations {
        let operator_len = equation.operands.len() - 1;
        let combo_add_mul_vec: Vec<Vec<Operator>> = combinations(vec![Operator::ADD, Operator::MUL], operator_len);
        // part1 ADD/MUL check
        for operators in combo_add_mul_vec {
            let test_result = apply_operators(&equation, &operators);
            if test_result == equation.result {
                if equation.operators.is_empty() {
                    result_part1 = result_part1 + equation.result;
                }
                equation.operators.push(operators.clone());
            }
        }
        // part2 ADD/MUL/CON check
        if equation.operators.is_empty() {
            let combo_add_mul_con_vec: Vec<Vec<Operator>> = combinations(vec![Operator::ADD, Operator::MUL, Operator::CON], operator_len);
            for operators in combo_add_mul_con_vec {
                let test_result = apply_operators(&equation, &operators);
                if test_result == equation.result {
                    if equation.operators.is_empty() {
                        result_part2 = result_part2 + equation.result;
                    }
                    equation.operators.push(operators.clone());
                }
            }
        }
    }
    result_part2 = result_part2 + result_part1;
    println!("Part1 = {}", result_part1);
    println!("Part2 = {}", result_part2);
}

fn combinations(operators: Vec<Operator>, size: usize) -> Vec<Vec<Operator>> {
    let mut result: Vec<Vec<Operator>> = Vec::new();
    for multi in itertools::repeat_n(operators.clone(), size).multi_cartesian_product() {
        let mut inner_vec: Vec<Operator> = Vec::new();
        for inner in multi {
            inner_vec.push(inner);
        }
        result.push(inner_vec);
    }
    result
}

fn apply_operators(equation: &Equation, operators: &Vec<Operator>) -> usize {
    let mut result: usize = equation.operands[0];
    let mut right_operator_index = 1;
    for operator in operators {
        result = apply_permution(result, &operator, equation.operands[right_operator_index]);
        right_operator_index = right_operator_index + 1;
    }
    result
}

fn apply_permution(left_operator: usize, operator: &Operator, right_operator: usize) -> usize {
    let mut value: usize = 0;
    if *operator == Operator::ADD {
        value = left_operator.add(right_operator);
    }
    if *operator == Operator::MUL {
        value = left_operator.mul(right_operator);
    }
    if *operator == Operator::CON {
        let mut concat = left_operator.to_string();
        concat.push_str(right_operator.to_string().as_str());
        value = concat.parse::<usize>().unwrap();
    }
    value
}

fn parse_equations(contents: String) -> Vec<Equation> {
    let lines = contents.lines();
    let mut equations = Vec::new();

    for line in lines {
        let equals_split: Vec<&str> = line.split(": ").collect();
        let left_side = equals_split[0].parse::<usize>().unwrap();
        let right_side: Vec<usize> = equals_split[1].split(" ").into_iter().map(|e| e.parse::<usize>().unwrap()).collect();
        let equation = Equation {result: left_side, operands: right_side, operators: Vec::new()};
        equations.push(equation);
    }
    equations
}

#[derive(Debug)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
    operators: Vec<Vec<Operator>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operator {
    ADD, // Addition
    MUL, // Multiplication
    CON, // Concatenation
}
