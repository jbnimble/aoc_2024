use std::env;
use std::fs;
use std::ops::Div;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let graphemes: Vec<&str> = contents.graphemes(true).filter(|&g| g != "\n").collect::<Vec<&str>>();
    let width: usize = contents.graphemes(true).into_iter().position(|g| g == "\n").unwrap();
    let height: usize = graphemes.iter().count().div(width);
    let agent_index: usize = graphemes.iter().position(|&g| g.eq("^")).unwrap();
    let obstacles: Vec<Location> = (0..graphemes.iter().count()).filter(|index| graphemes[*index] == "#").map(|index| from_index(index, width)).collect();
    let mut grid: Grid = Grid {width: width, height: height, obstacles: obstacles.clone()};
    let mut agent_tracking_part1 = Vec::new();
    let agent_start = Agent {location: from_index(agent_index, width), direction: Direction::U};
    let mut agent_option = Some(agent_start);

    while agent_option.is_some()  {
        if agent_option.is_some() {
            // find the unique locations for the agent, turning at 90 degrees until the agent goes off the grid
            if !agent_tracking_part1.iter().any(|a: &Agent| a.location == agent_option.unwrap().clone().location) {
                agent_tracking_part1.push(agent_option.unwrap().clone())
            }
            agent_option = next_move(agent_option.unwrap().clone(), &grid);
        }
    }
    println!("Part1 = {}", agent_tracking_part1.iter().count());

    let mut obstacle_loops: Vec<Location> = Vec::new();
    let tracks: Vec<Location> = agent_tracking_part1.iter().map(|e| e.location).collect();

    for n in 0..tracks.len() {
        let added_obstacle = tracks[n];
        agent_option = Some(agent_start);
        // skip existing obstacles, and agent starting position

        if !obstacles.contains(&added_obstacle) && agent_start.location.ne(&added_obstacle) {
            let mut obstacle_test: Vec<Location> = obstacles.clone();
            obstacle_test.push(added_obstacle);
            grid.obstacles = obstacle_test;
            let mut agent_tracking_part2 = Vec::new();

            while agent_option.is_some()  {
                if agent_option.is_some() {
                    // check for loops
                    if agent_tracking_part2.contains(&agent_option.unwrap().clone()) {
                        obstacle_loops.push(added_obstacle);
                        // println!("{} of {}\t{:?}", n, tracks.len(), added_obstacle);
                        break;
                    }
                    if !agent_tracking_part2.iter().any(|a: &Agent| a.location == agent_option.unwrap().clone().location) {
                        agent_tracking_part2.push(agent_option.unwrap().clone())
                    }
                    agent_option = next_move(agent_option.unwrap().clone(), &grid);
                }
            }
        }
    }

    println!("Part2 = {}", obstacle_loops.iter().count());
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    obstacles: Vec<Location>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Agent {
    location: Location,
    direction: Direction,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

// i = x + width * y
fn from_index(index: usize, width: usize) -> Location {
    Location {x: index % width, y: index / width}
}

// no move if going off grid, move at 90 degree direction if obstacle, otherwise move in direction
fn next_move(agent: Agent, grid: &Grid) -> Option<Agent> {
    match agent.direction {
        Direction::U => if agent.location.y == 0 {
                None
            } else if grid.obstacles.iter().any(|e| e.x == agent.location.x && e.y == agent.location.y - 1) {
                next_move(Agent {location: agent.location, direction: Direction::R}, grid)
            } else {
                Some(Agent {location: Location {x: agent.location.x, y: agent.location.y - 1}, direction: agent.direction})
            },
        Direction::D => if agent.location.y == grid.height - 1 {
                None
            } else if grid.obstacles.iter().any(|e| e.x == agent.location.x && e.y == agent.location.y + 1) {
                next_move(Agent {location: agent.location, direction: Direction::L}, grid)
            } else {
                Some(Agent {location: Location {x: agent.location.x, y: agent.location.y + 1}, direction: agent.direction})
            },
        Direction::L => if agent.location.x == 0 {
                None
            } else if grid.obstacles.iter().any(|e| e.x == agent.location.x - 1 && e.y == agent.location.y) {
                next_move(Agent {location: agent.location, direction: Direction::U}, grid)
            } else {
                Some(Agent {location: Location {x: agent.location.x - 1, y: agent.location.y}, direction: agent.direction})
            },
        Direction::R => if agent.location.x == grid.width - 1 {
                None
            } else if grid.obstacles.iter().any(|e| e.x == agent.location.x + 1 && e.y == agent.location.y) {
                next_move(Agent {location: agent.location, direction: Direction::D}, grid)
            } else {
                Some(Agent {location: Location {x: agent.location.x + 1, y: agent.location.y}, direction: agent.direction})
            },
    }
}
