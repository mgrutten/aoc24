use std::error::Error;
use std::fs;


pub fn match_word(puzzle: &Vec<Vec<char>>,
                  word: &Vec<char>,
                  x_index: usize,
                  y_index: usize,
                  x_step: i32,
                  y_step: i32) -> bool {
    let mut x = x_index as i32;
    let mut y = y_index as i32;
    for i in 0..word.len() {
        // Check for out-of-bounds
        if x < 0 || y < 0 || x >= puzzle.len() as i32 || y >= puzzle[0].len() as i32 {
            return false;
        }

        // Check for match
        if puzzle[x as usize][y as usize] != word[i] {
            return false;
        }

        // Increment
        x += x_step;
        y += y_step;
    }

    true
}

pub fn part1(puzzle: &Vec<Vec<char>>) {
    let word = vec!['X', 'M', 'A', 'S'];
    let directions = vec![-1, 0, 1];

    let mut match_count = 0;

    // Loop over each x, y location
    for x in 0..puzzle.len() {
        for y in 0..puzzle[0].len() {
            // Loop over each x, y direction
            for xd in directions.iter() {
                for yd in directions.iter() {
                    // Check for match
                    if match_word(puzzle, &word, x, y, *xd, *yd) {
                        match_count += 1
                    }
                }
            }
        }
    }

    println!("Part 1: {}", match_count);
}

pub fn part2(puzzle: &Vec<Vec<char>>) {
    let word = vec!['M', 'A', 'S'];
    //let directions = vec![-1, 0, 1];

    let mut match_count = 0;

    // Loop over each x, y location
    for x in 0..puzzle.len() {
        for y in 0..puzzle[0].len() {
            // Look for valid X-MAS combinations
            if (match_word(puzzle, &word, x, y, 1, 1) ||
                match_word(puzzle, &word, x + 2, y + 2, -1, -1)) &&
                (match_word(puzzle, &word, x + 2, y, -1, 1) ||
                    match_word(puzzle, &word, x, y + 2, 1, -1)) {
                match_count += 1
            }
        }
    }

    println!("Part 2: {}", match_count);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day04/day04.txt")?;

    // Parse into an array
    let puzzle: Vec<Vec<char>> = file_str.lines()
        .map(|line| line.chars().collect())
        .collect();

    part1(&puzzle);
    part2(&puzzle);

    Ok(())
}