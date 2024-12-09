use array2d::Array2D;
use std::error::Error;
use std::fs;


pub fn print_map(map: &Array2D<char>) {
    for row in map.rows_iter() {
        for element in row {
            print!("{}", element);
        }
        println!();
    }
}

pub fn find_guard(map: &Array2D<char>) -> (i32, i32) {
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            if !(map[(row, col)] == '.' || map[(row, col)] == '#') {
                return (row as i32, col as i32);
            }
        }
    }
    panic!("No guard found");
}

pub fn move_guard(map: &mut Array2D<char>,
                  location: &(i32, i32),
                  step: &(i32, i32)) -> Option<(i32, i32)> {
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
        map[(next_row as usize, next_col as usize)] = 'X';
        new_location = (next_row, next_col);
    }

    Some(new_location)
}

pub fn turn_right(step: &(i32, i32)) -> (i32, i32) {
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

pub fn part1(orig_map: &Array2D<char>) {
    let mut map = orig_map.clone();

    let mut location = find_guard(&map);
    map[(location.0 as usize, location.1 as usize)] = 'X';

    // Keep moving and turning right
    let mut step = (-1, 0);
    loop {
        match move_guard(&mut map, &location, &step) {
            Some(value) => location = value,
            None => break,
        }
        step = turn_right(&step);
    }

    // Count the number of 'X'
    let visits: i32 = map.elements_row_major_iter()
        .map(|e| if *e == 'X' { 1 } else { 0 })
        .sum();
    println!("Part 1: {}", visits);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day06/day06.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let map = Array2D::from_rows(&map_vec).unwrap();

    part1(&map);

    Ok(())
}