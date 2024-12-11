use std::collections::HashMap;
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


fn count_stones(stones: &Vec<u64>, blink_count: u32) {
    // Histogram of stones
    let mut stone_hist = HashMap::new();
    for stone in stones {
        *stone_hist.entry(*stone).or_insert(0) += 1;
    }

    for _ in 0..blink_count {
        let mut new_stone_hist = HashMap::new();
        for (stone, count) in stone_hist.into_iter() {
            for new_stone in apply_rule(stone) {
                *new_stone_hist.entry(new_stone).or_insert(0) += count;
            }
        }
        stone_hist = new_stone_hist;
    }
    
    println!("Stone count: {}", stone_hist.values().sum::<u64>());
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day11/day11.txt")?;

    // Numbers on stones
    let stones = file_str.split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    count_stones(&stones, 25);
    count_stones(&stones, 75);

    Ok(())
}