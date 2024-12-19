use std::collections::HashMap;
use std::error::Error;
use std::fs;


fn match_design(design: &[char], patterns: &Vec<Vec<char>>) -> bool {
    let mut stack = Vec::new();
    stack.push(0_usize);

    //println!("{:?}", design);

    while let Some(idx) = stack.pop() {
        //println!("{}/{}", idx, design.len());
        if idx == design.len() {
            return true;
        }

        for p in patterns {
            //println!("p: {:?}", p) ;
            let len = p.len();
            if design.len() >= idx + len {
                //println!("{:?} : {:?}", &design[idx..idx + len], p) ;
                if design[idx..idx + len] == *p {
                    stack.push(idx + len);
                }
            }
        }
    }

    false
}


fn count_designs(design: &[char], patterns: &Vec<Vec<char>>) -> u64 {
    let design_str = design.iter().collect::<String>();
    let design_patterns = patterns.iter()
        .filter(|p| design_str.contains(p.iter().collect::<String>().as_str()))
        .collect::<Vec<_>>();

    let mut memo: HashMap<usize, u64> = HashMap::new();
    count_recursion(design, &design_patterns, 0, &mut memo)
}

fn count_recursion(design: &[char],
                   design_patterns: &Vec<&Vec<char>>,
                   idx: usize,
                   memo: &mut HashMap<usize, u64>) -> u64 {
    if idx == design.len() {
        return 1;
    }

    if let Some(&cached_result) = memo.get(&idx) {
        return cached_result;
    }

    let mut count = 0;
    for p in design_patterns.iter() {
        let len = p.len();
        if design.len() >= idx + len && design[idx..idx + len] == **p {
            count += count_recursion(design, design_patterns, idx + len, memo);
        }
    }

    memo.insert(idx, count);

    count
}

fn part1(patterns: &Vec<Vec<char>>, designs: &Vec<Vec<char>>) {
    let mut count = 0;
    for design in designs {
        if match_design(design, patterns) {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
}


fn part2(patterns: &Vec<Vec<char>>, designs: &Vec<Vec<char>>) {
    let count = designs.iter()
        .fold(0, |acc, design| acc + count_designs(design, patterns));

    println!("Part 2: {}", count);
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day19/day19.txt")?;

    let mut lines = file_str.lines();
    let patterns = lines.next().unwrap()
        .split(", ")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    lines.next();
    let designs = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&patterns, &designs);
    part2(&patterns, &designs);

    Ok(())
}