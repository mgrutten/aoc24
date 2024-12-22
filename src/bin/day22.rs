use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn gen_next(num: u64) -> u64 {
    let mask = 16777215; // 2^24 - 1
    let next1 = ((num << 6) ^ num) & mask;
    let next2 = ((next1 >> 5) ^ next1) & mask;

    ((next2 << 11) ^ next2) & mask
}


fn part1(secrets: &Vec<u64>) {
    let mut total = 0;
    for secret in secrets {
        let mut num = *secret;
        for _ in 0..2000 {
            num = gen_next(num);
        }
        total += num;
    }
    println!("Part 1: {}", total);
}


fn part2(secrets: &Vec<u64>) {
    let mut patterns = HashMap::new();

    for secret in secrets {
        let mut digits = Vec::new();
        let mut num = *secret;
        digits.push((num % 10) as i8);
        for _ in 0..2000 {
            num = gen_next(num);
            digits.push((num % 10) as i8);
        }
        let diffs = digits.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

        let mut unique = HashSet::new();
        for idx in 0..=diffs.len() - 4 {
            let seq = &diffs[idx..idx + 4];
            if unique.insert(seq.to_vec()) {
                *patterns.entry(seq.to_vec()).or_insert(0) += digits[idx + 4] as u64;
            }
        }
    }

    let max_pattern = patterns.iter()
        .max_by_key(|&(_, count)| count).unwrap();

    println!("Part 2: {:?}", max_pattern);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day22/day22.txt")?;
    
    let secrets = file_str.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    part1(&secrets);
    part2(&secrets);

    Ok(())
}
