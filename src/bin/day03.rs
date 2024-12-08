use std::error::Error;
use std::fs;
use regex::Regex;

pub fn part1(code: &str) {
    // Form regex matching mul
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Sum up multiplications
    let sum = regex.captures_iter(&code)
        .map(|c| c.extract())
        .fold(0, |acc, (_, [a, b])| acc + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap());

    println!("Part 1: {}", sum);
}


pub fn part2(code: &str) {
    // Form regex's for our patterns
    let do_regex = Regex::new(r"^do\(\)").unwrap();
    let dont_regex = Regex::new(r"^don't\(\)").unwrap();
    let mul_regex = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Loop over code looking for matches
    let mut do_mul = true;
    let mut sum = 0;
    for i in 0..code.len() {
        if !do_mul && do_regex.is_match(&code[i..]) {
            do_mul = true;
        } else if do_mul && dont_regex.is_match(&code[i..]) {
            do_mul = false;
        } else if do_mul && mul_regex.is_match(&code[i..]) {
            let (_, [a, b]) = mul_regex.captures(&code[i..]).unwrap().extract();
            sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }

    println!("Part 2: {}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day03/day03.txt")?;

    part1(&file_str);
    part2(&file_str);

    Ok(())
}