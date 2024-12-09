use array2d::Array2D;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn find_guard(map: &Array2D<char>) -> (i32, i32) {
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            if !(map[(row, col)] == '.' || map[(row, col)] == '#') {
                return (row as i32, col as i32);
            }
        }
    }
    panic!("No guard found");
}

fn move_guard(map: &Array2D<char>,
                  location: &(i32, i32),
                  step: &(i32, i32),
                  visited: &mut HashSet<(i32, i32)>) -> Option<(i32, i32)> {
    let mut new_location = location.clone();
    loop {
        let next_row = new_location.0 + step.0;
        let next_col = new_location.1 + step.1;
        if next_row < 0 || next_row >= map.num_rows() as i32 ||
            next_col < 0 || next_col >= map.num_columns() as i32 {
            return None;
        }
        if map[(next_row as usize, next_col as usize)] == '#' {
            break;
        }
        new_location = (next_row, next_col);
        visited.insert(new_location);
    }

    Some(new_location)
}


enum CycleResult {
    OutOfBounds,
    NewLocation((i32, i32)),
    Cycle,
}


fn move_guard_with_step(map: &Array2D<char>,
                            location: &(i32, i32),
                            step: &(i32, i32),
                            visited: &mut HashSet<((i32, i32), (i32, i32))>) -> CycleResult {
    let mut new_location = location.clone();
    loop {
        let next_row = new_location.0 + step.0;
        let next_col = new_location.1 + step.1;
        if next_row < 0 || next_row >= map.num_rows() as i32 ||
            next_col < 0 || next_col >= map.num_columns() as i32 {
            return CycleResult::OutOfBounds;
        }
        if map[(next_row as usize, next_col as usize)] == '#' {
            break;
        }
        new_location = (next_row, next_col);
        if visited.contains(&(new_location, *step)) {
            return CycleResult::Cycle;
        }
        visited.insert((new_location, *step));
    }

    CycleResult::NewLocation(new_location)
}


fn test_for_cycle(map: &Array2D<char>,
                      initial_location: &(i32, i32)) -> bool {
    // Initial location and direction
    let mut location = initial_location.clone();
    let mut step = (-1, 0);

    // Set of visited locations
    let mut visited = HashSet::new();
    visited.insert((location, step));

    // Keep moving and turning right
    loop {
        match move_guard_with_step(&map, &location, &step, &mut visited) {
            CycleResult::Cycle => return true,
            CycleResult::NewLocation(value) => location = value,
            CycleResult::OutOfBounds => break,
        }
        step = turn_right(&step);
    }

    false
}

fn turn_right(step: &(i32, i32)) -> (i32, i32) {
    if *step == (-1, 0) {
        (0, 1)
    } else if *step == (1, 0) {
        (0, -1)
    } else if *step == (0, -1) {
        (-1, 0)
    } else if *step == (0, 1) {
        (1, 0)
    } else {
        panic!("Wrong step!");
    }
}


fn find_locations(map: &Array2D<char>,
                      initial_location: &(i32, i32)) -> HashSet<(i32, i32)> {
    // Initial location
    let mut location = initial_location.clone();

    // Set of visited locations
    let mut visited = HashSet::new();
    visited.insert(location);

    // Keep moving and turning right
    let mut step = (-1, 0);
    loop {
        match move_guard(&map, &location, &step, &mut visited) {
            Some(value) => location = value,
            None => break,
        }
        step = turn_right(&step);
    }

    visited
}

fn part1(map: &Array2D<char>) {
    // Initial location
    let initial_location = find_guard(&map);

    // Set of visited locations
    let visited = find_locations(&map, &initial_location);

    // The number of visited locations
    println!("Part 1: {}", visited.len());
}

fn part2(map: &Array2D<char>) {
    // Initial location
    let initial_location = find_guard(&map);

    // Set of visited locations
    let visited = find_locations(&map, &initial_location);

    // Try each original visited location for new obstacle location
    let mut cycle_count = 0;
    for (row, col) in visited {
        if (row, col) != initial_location {
            let mut test_map = map.clone();
            test_map[(row as usize, col as usize)] = '#';
            if test_for_cycle(&test_map, &initial_location) { cycle_count += 1 }
        }
    }

    // The number locations that result in a cycle
    println!("Part 2: {}", cycle_count);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day06/day06.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let map = Array2D::from_rows(&map_vec).unwrap();

    part1(&map);
    part2(&map);

    Ok(())
}