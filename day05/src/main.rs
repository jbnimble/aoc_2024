use std::env;
use std::fs;
use std::ops::Div;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Failed to read file");
    let data_str: Vec<&str> = contents.split("\n\n").collect();
    let page_orders = parse_order(data_str[0]);
    let manuals = parse_manuals(data_str[1]);

    let mut total_part1: i32 = 0;
    let mut total_part2: i32 = 0;

    for manual in manuals {
        let checks = manual_check(&manual, &page_orders);
        if checks.is_empty() {
            // find correctly ordered manuals, add middle page values for part1
            let middle = manual[manual.len().div(2)] as i32;
            total_part1 = total_part1 + middle;
        } else {
            // find incorrectly ordered manuals, fix the order, add middle page values for part2
            let mut manual_wip = manual.clone();
            let mut wip_check = manual_check(&manual_wip, &page_orders);
            while !wip_check.is_empty() {
                for check in wip_check {
                    let index_a = manual_wip.iter().position(|e| e.eq(&check[0])).unwrap();
                    let index_b = manual_wip.iter().position(|e| e.eq(&check[1])).unwrap();
                    manual_wip.swap(index_a, index_b);
                }
                wip_check = manual_check(&manual_wip, &page_orders);
            }
            if !wip_check.is_empty() {
                println!("failed manual {:?}\nchecks {:?}", &manual_wip, wip_check);
            } else {
                let middle = manual_wip[manual_wip.len().div(2)] as i32;
                total_part2 = total_part2 + middle;
            }
        }
    }

    println!("Part1 = {total_part1}");
    println!("Part2 = {total_part2}");
}

fn manual_check(manual: &Vec<usize>, page_orders: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for page_i in 0..manual.len() {
        for page_n in 0..manual.len() {
            let mut check = Vec::new();
            if page_n < page_i {
                // check page_n rules
                check = order_check(manual[page_n], manual[page_i], page_orders);
            }
            if page_i < page_n {
                // check page_i rules
                check = order_check(manual[page_i], manual[page_n], page_orders);
            }
            if !check.is_empty() && !result.contains(&check){
                result.push(check);
            }
        }
    }
    result
}

// check if 'b' comes before 'a' in any page order rules
fn order_check(page_a: usize, page_b: usize, page_orders: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut result = Vec::new();
    let page_order = &page_orders[page_b];
    if page_order.contains(&page_a) {
        result.push(page_b);
        result.push(page_a);
    }
    result
}

fn parse_order(data: &str) -> Vec<Vec<usize>> {
    let mut lines: Vec<&str> = data.lines().collect();
    lines.sort();

    let mut result:Vec<Vec<usize>> = Vec::new();
    for _ in 0..100 {
        result.push(Vec::new());
    }
    // data is of the form KEY|VAL
    for line in lines {
        let order:Vec<usize> = line.split("|").map(|s| s.parse::<usize>().unwrap()).collect();
        result[order[0]].push(order[1]);
    }
    result
}

fn parse_manuals(data: &str) -> Vec<Vec<usize>> {
    let mut result:Vec<Vec<usize>> = Vec::new();
    for line in data.lines() {
        result.push(line.split(",").map(|s| s.parse::<usize>().unwrap()).collect());
    }
    result
}
