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
            //println!("{}", num);
        }
        total += num;
    }
    println!("Part 1: {}", total);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day22/day22.txt")?;


    let secrets = file_str.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    part1(&secrets);
    
    Ok(())
}
