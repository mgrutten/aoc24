use array2d::Array2D;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::{fmt, fs};

#[derive(Debug, Clone, PartialEq)]
enum MapContent {
    Wall,
    Empty,
    Start,
    End,
    Visited,
}

impl fmt::Display for MapContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapContent::Wall => write!(f, "#"),
            MapContent::Start => write!(f, "S"),
            MapContent::End => write!(f, "E"),
            MapContent::Empty => write!(f, "."),
            MapContent::Visited => write!(f, "O"),
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


#[derive(Debug, Clone, Eq, PartialEq)]
struct CostState {
    cost: u64,
    state: State,
}

impl Ord for CostState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for CostState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn collect_visited_locations(
    location: &(usize, usize),
    previous: &HashMap<(usize, usize), Vec<(usize, usize)>>,
    visited_locations: &mut HashSet<(usize, usize)>,
) {
    let mut stack = vec![location.clone()];
    let mut visited = HashSet::new();

    while let Some(current) = stack.pop() {
        if visited.insert(current) {
            visited_locations.insert(current);

            if let Some(prev_states) = previous.get(&current) {
                for prev in prev_states.iter() {
                    stack.push(prev.clone());
                }
            }
        }
    }
}

fn explore(map: &Array2D<MapContent>,
           start_state: &State,
           min_cost: &mut u64,
           visited_locations: &mut HashSet<(usize, usize)>) {
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut previous = HashMap::<(usize, usize), Vec<(usize, usize)>>::new();

    distances.insert(start_state.clone(), 0);
    heap.push(CostState { cost: 0, state: start_state.clone() });

    while let Some(CostState { cost: current_cost, state: current_state }) = heap.pop() {
        if current_cost > *min_cost {
            continue;
        }

        if map[current_state.location] == MapContent::End {
            if current_cost < *min_cost {
                *min_cost = current_cost;
                visited_locations.clear();
            }

            if current_cost == *min_cost {
                collect_visited_locations(&current_state.location, &previous, visited_locations);
            }
            continue;
        }

        if let Some(&dist) = distances.get(&current_state) {
            if current_cost > dist {
                continue;
            }
        }

        let new_location = match current_state.direction {
            Direction::East => (current_state.location.0, current_state.location.1 + 1),
            Direction::West => (current_state.location.0, current_state.location.1 - 1),
            Direction::North => (current_state.location.0 - 1, current_state.location.1),
            Direction::South => (current_state.location.0 + 1, current_state.location.1),
        };
        let forward = State {
            location: new_location,
            direction: current_state.direction.clone(),
        };

        if map[forward.location] == MapContent::Empty || map[forward.location] == MapContent::End {
            let new_cost = current_cost + 1;
            if new_cost <= *distances.get(&forward).unwrap_or(&u64::MAX) {
                distances.insert(forward.clone(), new_cost);
                previous.entry(forward.location).or_insert_with(Vec::new).push(current_state.location);
                heap.push(CostState { cost: new_cost, state: forward });
            }
        }

        // Turn left
        {
            let new_direction = match current_state.direction {
                Direction::East => Direction::North,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
                Direction::South => Direction::East,
            };
            let new_cost = current_cost + 1000;
            let new_state = State {
                location: current_state.location,
                direction: new_direction,
            };
            if new_cost <= *distances.get(&new_state).unwrap_or(&u64::MAX) {
                distances.insert(new_state.clone(), new_cost);
                heap.push(CostState { cost: new_cost, state: new_state });
            }
        }

        // Turn Right
        {
            let new_direction = match current_state.direction {
                Direction::East => Direction::South,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
                Direction::South => Direction::West,
            };
            let new_cost = current_cost + 1000;
            let new_state = State {
                location: current_state.location,
                direction: new_direction,
            };
            if new_cost <= *distances.get(&new_state).unwrap_or(&u64::MAX) {
                distances.insert(new_state.clone(), new_cost);
                heap.push(CostState { cost: new_cost, state: new_state });
            }
        }
    }
}

fn part1(map: &Array2D<MapContent>) {
    let start = State {
        location: find_start(map),
        direction: Direction::East,
    };

    let mut min_cost = u64::MAX;
    let mut visited_locations = HashSet::new();
    explore(map, &start, &mut min_cost, &mut visited_locations);
    println!("Part 1: {}, {}", min_cost, visited_locations.len());

    //let mut mut_map = map.clone();
    //let check_empty = visited_locations.iter()
    //    .all(|loc| map[*loc] != MapContent::Wall);
    //println!("Part 1: {}", check_empty);

    //visited_locations.iter()
    //    .for_each(|loc| mut_map[*loc] = MapContent::Visited);
    //print_map(&mut_map);
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day16/day16-example.txt")?;

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

    part1(&map);

    Ok(())
}