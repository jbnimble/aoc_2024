use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let contents: String = fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("Failed to read file");

    let data: Map2D = parse_data(&contents);
    // let part1_result: usize = calculate_part1(&data);
    // println!("Part1 = {}", part1_result); // should be 293 apparently, instead getting 298

    let part2_result: usize = calculate_part2(&data);
    println!("Part2 = {}", part2_result); // 464 too low, 562 is too low
}

fn calculate_part1(data: &Map2D) -> usize {
    let mut result: HashSet<Point2D> = HashSet::new();

    for key in data.points.keys() {
        let points: &Vec<Point2D> = data.points.get(key).unwrap();
        for i in 0..points.len() {
            for j in 0..points.len() {
                let point_a: &Point2D = &points[i];
                let point_b: &Point2D = &points[j];
                if point_a != point_b {
                    let diff: Point2D = point_a.sub(&point_b);
                    let antinode_a: Point2D = point_a.sub(&diff);
                    let antinode_b: Point2D = point_b.add(&diff);
                    result.insert(antinode_a.clone());
                    result.insert(antinode_b.clone());
                }
            }
        }
    }
    result.iter().filter(|p|is_inside_map(&p, &data)).count()
}

fn calculate_part2(data: &Map2D) -> usize {
    let mut result: HashSet<Point2D> = HashSet::new();

    for key in data.points.keys() {
        let points: &Vec<Point2D> = data.points.get(key).unwrap();
        for i in 0..points.len() {
            for j in 0..points.len() {
                let point_a: &Point2D = &points[i];
                let point_b: &Point2D = &points[j];
                if point_a != point_b {
                    // add antennas
                    result.insert(point_a.clone());
                    result.insert(point_b.clone());
                    let diff: Point2D = point_a.sub(&point_b);
                    // handle harmonics
                    let mut antinode_a: Point2D = point_a.sub(&diff);
                    let mut rolling_diff_a = point_a.clone();
                    while is_inside_map(&antinode_a.clone(), data) {
                        result.insert(antinode_a.clone());
                        let temp_diff = antinode_a.clone();
                        antinode_a = antinode_a.sub(&rolling_diff_a);
                        rolling_diff_a = temp_diff;
                        println!("{}_a => size {} i={} j={} an={:?} diff={:?}", key, result.len(), i, j, antinode_a, rolling_diff_a);
                    }
                    println!("");
                    let mut antinode_b: Point2D = point_b.add(&diff);
                    let mut rolling_diff_b = point_b.clone();
                    while is_inside_map(&antinode_b.clone(), data) {
                        result.insert(antinode_b.clone());
                        let temp_diff = antinode_b.clone();
                        antinode_b = antinode_b.add(&rolling_diff_b);
                        rolling_diff_b = temp_diff;
                        println!("{}_b => size {} i={} j={} an={:?} diff={:?}", key, result.len(), i, j, antinode_b, rolling_diff_b);
                    }
                    println!("");
                }
            }
        }
    }
    result.iter().filter(|p|is_inside_map(&p, &data)).count()
}

fn parse_data(contents: &String) -> Map2D {
    let graphemes: Vec<&str> = contents.graphemes(true).filter(|&g| g != "\n").collect::<Vec<&str>>();
    let width = contents.clone().graphemes(true).into_iter().position(|g| g == "\n").unwrap() as isize;
    let mut name_points_map: HashMap<char, Vec<Point2D>> = HashMap::new(); // usage simulates MultiMap
    let re = Regex::new("([a-z]|[A-Z]|[0-9])").unwrap(); // lowercase or uppercase or digit
    for index in 0..graphemes.len() {
        let g = graphemes[index];
        if re.is_match(g) {
            let g_char = g.chars().next().expect("Failed to get character");
            name_points_map.entry(g_char).or_insert(Vec::new());
            name_points_map.entry(g_char).and_modify(|v| v.push(Point2D {x: index as isize % width, y: index as isize / width}));
        }
    }
    Map2D {width: width, height: graphemes.len() as isize / width, points: name_points_map}
}

fn is_inside_map(point: &Point2D, map: &Map2D) -> bool {
    point.x >= 0 && point.y >= 0 && point.x < map.width && point.y < map.height
}

struct Map2D {
    width: isize,
    height: isize,
    points: HashMap<char, Vec<Point2D>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point2D {
    x: isize,
    y: isize,
}

impl Point2D {
    fn add(&self, b: &Point2D) -> Point2D {
        Point2D {x: b.x + self.x, y: b.y + self.y}
    }
    fn sub(&self, b: &Point2D) -> Point2D {
        Point2D {x: b.x - self.x, y: b.y - self.y}
    }
}
