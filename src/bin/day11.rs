use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn num_digits(stone: u64) -> u32 {
    stone.ilog10() + 1
}

fn apply_rule(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else if num_digits(stone) % 2 == 0 {
        let half_digit_factor = 10_u64.pow(num_digits(stone) / 2);
        vec![stone % half_digit_factor, stone / half_digit_factor]
    } else {
        vec![stone * 2024]
    }
}

fn part1(stones: &Vec<u64>) {
    let mut new_stones = stones.clone();
    for _ in 0..25 {
        new_stones = new_stones.into_iter()
            .map(|stone| apply_rule(stone))
            .flat_map(|v| v.into_iter())
            .collect::<Vec<u64>>();
    }

    println!("Part 1: {}", new_stones.len());
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day11/day11.txt")?;

    // Numbers on stones
    let stones = file_str.split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    part1(&stones);

    Ok(())
}