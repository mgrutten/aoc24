use std::error::Error;
use std::fs;

fn find_match(value: u64, target: u64, nums: &[u64]) -> bool {

    // Check against the target if we're at the end of the equation
    if nums.is_empty() {
        return value == target;
    }

    // Only check for multiply if we're within the target
    let test_mult = if target >= value * nums[0] {
        find_match(value * nums[0], target, &nums[1..])
    } else {
        false
    };

    // Recursive depth-first search
    test_mult || find_match(value + nums[0], target, &nums[1..])
}

fn part1(puzzle: &Vec<(u64, Vec<u64>)>) {
    let mut calibration = 0;
    for (value, nums) in puzzle {
        if find_match(nums[0], *value, &nums[1..]) {
            calibration += value;
        }
    }

    println!("Part 1: {}", calibration);
}


fn concat_u64(left: u64, right: u64) -> u64 {
    let digit_count = right.ilog10() + 1;

    left * 10_u64.pow(digit_count) + right
}


fn find_match_concat(value: u64, target: u64, nums: &[u64]) -> bool {

    // Check against the target if we're at the end of the equation
    if nums.is_empty() {
        return value == target;
    }

    // Only check for multiply if we're within the target
    let test_mult = if target >= value * nums[0] {
        find_match_concat(value * nums[0], target, &nums[1..])
    } else {
        false
    };

    // Recursive depth-first search
    test_mult ||
        find_match_concat(value + nums[0], target, &nums[1..]) ||
        find_match_concat(concat_u64(value, nums[0]), target, &nums[1..])
}


fn part2(puzzle: &Vec<(u64, Vec<u64>)>) {
    let mut calibration = 0;
    for (value, nums) in puzzle {
        if find_match_concat(nums[0], *value, &nums[1..]) {
            calibration += value;
        }
    }

    println!("Part 2: {}", calibration);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day07/day07.txt")?;

    let puzzle = file_str.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(test_value, nums)| (
            test_value.parse::<u64>().unwrap(),
            nums.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        ))
        .collect::<Vec<(u64, Vec<u64>)>>();

    part1(&puzzle);
    part2(&puzzle);

    Ok(())
}