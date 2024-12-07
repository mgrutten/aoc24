use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn part1(col1: &Vec<i32>, col2: &Vec<i32>) {
    // Sort column data
    let mut col1_sorted = col1.clone();
    col1_sorted.sort();

    let mut col2_sorted = col2.clone();
    col2_sorted.sort();

    // Add up differences
    let dist = col1_sorted.iter().zip(col2_sorted.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("Part 1: {}", dist);
}


pub fn part2(col1: &Vec<i32>, col2: &Vec<i32>) {
    // Count up distinct values in col2
    let mut col2_map = HashMap::new();
    col2.iter()
        .for_each(|val| *col2_map.entry(val).or_insert(0) += 1);

    // Sum up matches from col1
    let similarity = col1.iter()
        .fold(0, |acc, val| acc + val * col2_map.get(val).unwrap_or(&0));

    println!("Part 2: {:?}", similarity);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day01/day01.txt")?;

    // Parse into columns
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in file_str.lines() {
        let pair = line.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        col1.push(pair[0]);
        col2.push(pair[1]);
    }

    part1(&col1, &col2);
    part2(&col1, &col2);

    Ok(())
}