use array2d::Array2D;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::{fmt, fs};

#[derive(Debug, Clone, PartialEq)]
enum MapContent {
    Wall,
    Empty,
    Start,
    End,
}

impl fmt::Display for MapContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapContent::Wall => write!(f, "#"),
            MapContent::Start => write!(f, "S"),
            MapContent::End => write!(f, "E"),
            MapContent::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct State {
    location: (usize, usize),
    direction: Direction,
}

fn print_map(map: &Array2D<MapContent>) {
    for row in map.rows_iter() {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
}

fn find_start(map: &Array2D<MapContent>) -> (usize, usize) {
    map.enumerate_row_major()
        .find(|(_, val)| **val == MapContent::Start)
        .unwrap().0
}

/*
fn explore(map: &Array2D<MapContent>,
           state: &State,
           visited: &mut HashSet<State>,
           min_cost: &mut u64,
           current_cost: &mut u64,
           current_path: &mut Vec<State>) {
    visited.insert(state.clone());

    // println!("{:?}", current_path);

    if map[state.location] == MapContent::End {
        println!("got to end {}", current_cost);
        if current_cost < min_cost {
            *min_cost = *current_cost;
        }
    } else {
        // Try each move
        let forward = match state.direction {
            Direction::East => State {
                location: (state.location.0, state.location.1 + 1),
                direction: state.direction.clone(),
            },
            Direction::West => State {
                location: (state.location.0, state.location.1 - 1),
                direction: state.direction.clone(),
            },
            Direction::North => State {
                location: (state.location.0 - 1, state.location.1),
                direction: state.direction.clone(),
            },
            Direction::South => State {
                location: (state.location.0 + 1, state.location.1),
                direction: state.direction.clone(),
            },
        };

        // Try moving forward
        if !visited.contains(&forward) &&
            (map[forward.location] == MapContent::Empty || map[forward.location] == MapContent::End) {
            *current_cost += 1;
            if current_cost < min_cost {
                current_path.push(forward.clone());
                explore(map, &forward, visited, min_cost, current_cost, current_path);
                current_path.pop();
            }
            *current_cost -= 1;
        }

        // Only turn once
        if current_path.len() < 2 ||
            current_path[current_path.len() - 2].location != current_path[current_path.len() - 1].location {
            // Turn left
            let new_direction = match state.direction {
                Direction::East => Direction::North,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
                Direction::South => Direction::East,
            };
            *current_cost += 1000;
            if current_cost < min_cost {
                let new_state = State {
                    location: state.location,
                    direction: new_direction,
                };
                current_path.push(new_state.clone());
                explore(map, &new_state, visited, min_cost, current_cost, current_path);
                current_path.pop();
            }
            *current_cost -= 1000;

            // Turn Right
            let new_direction = match state.direction {
                Direction::East => Direction::South,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
                Direction::South => Direction::West,
            };
            *current_cost += 1000;
            if current_cost < min_cost {
                let new_state = State {
                    location: state.location,
                    direction: new_direction,
                };
                current_path.push(new_state.clone());
                explore(map, &new_state, visited, min_cost, current_cost, current_path);
                current_path.pop();
            }
            *current_cost -= 1000;
        }
    }

    visited.remove(state);
}

 */

fn explore(
    map: &Array2D<MapContent>,
    start: &State) -> u64 {
    let mut visited = HashMap::new();
    let mut min_cost = u64::MAX;

    let mut stack = VecDeque::new();
    stack.push_back((start.clone(), 0, vec![start.clone()]));

    while let Some((current_state, current_cost, current_path)) = stack.pop_back() {
        if let Some(&prev_cost) = visited.get(&current_state) {
            if current_cost >= prev_cost {
                continue;
            }
        }
        visited.insert(current_state.clone(), current_cost);

        if map[current_state.location] == MapContent::End {
            if current_cost < min_cost {
                min_cost = current_cost;
            }
        } else {
            let forward = match current_state.direction {
                Direction::East => State {
                    location: (current_state.location.0, current_state.location.1 + 1),
                    direction: current_state.direction.clone(),
                },
                Direction::West => State {
                    location: (current_state.location.0, current_state.location.1 - 1),
                    direction: current_state.direction.clone(),
                },
                Direction::North => State {
                    location: (current_state.location.0 - 1, current_state.location.1),
                    direction: current_state.direction.clone(),
                },
                Direction::South => State {
                    location: (current_state.location.0 + 1, current_state.location.1),
                    direction: current_state.direction.clone(),
                },
            };

            if map[forward.location] == MapContent::Empty || map[forward.location] == MapContent::End {
                if current_cost + 1 < min_cost {
                    let mut new_path = current_path.clone();
                    new_path.push(forward.clone());
                    stack.push_back((forward, current_cost + 1, new_path));
                }
            }

            if current_path.len() < 2
                || current_path[current_path.len() - 2].location
                != current_path[current_path.len() - 1].location {

                // Turn left
                let new_direction = match current_state.direction {
                    Direction::East => Direction::North,
                    Direction::West => Direction::South,
                    Direction::North => Direction::West,
                    Direction::South => Direction::East,
                };
                if current_cost + 1000 < min_cost {
                    let new_state = State {
                        location: current_state.location,
                        direction: new_direction,
                    };
                    let mut new_path = current_path.clone();
                    new_path.push(new_state.clone());
                    stack.push_back((new_state, current_cost + 1000, new_path));
                }

                // Turn Right
                let new_direction = match current_state.direction {
                    Direction::East => Direction::South,
                    Direction::West => Direction::North,
                    Direction::North => Direction::East,
                    Direction::South => Direction::West,
                };
                if current_cost + 1000 < min_cost {
                    let new_state = State {
                        location: current_state.location,
                        direction: new_direction,
                    };
                    let mut new_path = current_path.clone();
                    new_path.push(new_state.clone());
                    stack.push_back((new_state, current_cost + 1000, new_path));
                }

            }
        }

        // visited.remove(&current_state);
    }

    min_cost
}


fn part1(map: &Array2D<MapContent>) {
    let start = State {
        location: find_start(map),
        direction: Direction::East,
    };

    //let mut min_cost = u64::MAX;
    //let mut current_cost = 0;
    //let mut current_path = vec![start.clone()];
    let min_cost = explore(map, &start);
    println!("Part 1: {}", min_cost);
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day16/day16.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars()
            .map(|c| match c {
                '#' => MapContent::Wall,
                'S' => MapContent::Start,
                'E' => MapContent::End,
                '.' => MapContent::Empty,
                _ => unreachable!()
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = Array2D::from_rows(&map_vec).unwrap();
    //print_map(&map);

    part1(&map);

    Ok(())
}