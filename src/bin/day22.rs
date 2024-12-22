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
        let mut num = *secret;

        let mut unique = HashSet::new();
        let mut diffs = [0_i8; 4];
        for idx in 0..2000 {
            let last = (num % 10) as i8;
            num = gen_next(num);

            diffs.rotate_left(1);
            let curr = (num % 10) as i8;
            diffs[3] = curr - last;
            if idx >= 3 {
                if unique.insert(diffs) {
                    *patterns.entry(diffs).or_insert(0) += curr as u64;
                }
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
