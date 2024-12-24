use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn decimal_value(binary: &[bool]) -> u64 {
    let mut val = 0;
    for (i, &bit) in binary.iter().rev().enumerate() {
        if bit {
            val += 2_u64.pow(i as u32);
        }
    }

    val
}

fn decimal_to_bits(n: u64) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..46 {
        let bit = ((n >> i) & 1) == 1;
        bits.push(bit);
    }
    bits.reverse();
    bits
}

fn extract_value(values: &HashMap<String, bool>, query: char) -> u64 {
    let mut sorted_values = values.iter()
        .filter(|&(key, _)| key.chars().collect::<Vec<_>>()[0] == query)
        .collect::<Vec<_>>();
    sorted_values.sort_by_key(|&(key, _)| Reverse(key));
    let bin_values = sorted_values.iter()
        .map(|&(_, val)| val)
        .cloned()
        .collect::<Vec<_>>();

    decimal_value(&bin_values)
}

fn part1(initial_values: &HashMap<String, bool>, operations: &HashMap<[String; 3], String>) -> u64 {
    let targets = operations.keys()
        .map(|key| key[2].clone())
        .filter(|v| v.chars().collect::<Vec<_>>()[0] == 'z')
        .collect::<HashSet<_>>();

    let mut values = initial_values.clone();
    while !values.keys().cloned().collect::<HashSet<_>>().is_superset(&targets) {
        for operator in operations.keys() {
            if values.contains_key(&operator[0]) && values.contains_key(&operator[1]) {
                let left = values.get(&operator[0]).unwrap();
                let right = values.get(&operator[1]).unwrap();
                let result = match operations.get(operator).unwrap().as_str() {
                    "AND" => left & right,
                    "OR" => left | right,
                    "XOR" => left ^ right,
                    _ => unreachable!(),
                };
                values.insert(operator[2].clone(), result);
            }
        }
    }

    // Extract values from target
    extract_value(&values, 'z')
}

fn read_file(
    file_name: &str
) -> Result<(HashMap<String, bool>, HashMap<[String; 3], String>), Box<dyn Error>> {
    let file_str: String = fs::read_to_string(file_name)?;
    let mut lines = file_str.lines();

    let mut initial_values = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let lr = line.split(": ").collect::<Vec<&str>>();
        initial_values.insert(lr[0].to_string(), lr[1] == "1");
    }

    let mut operations = HashMap::new();
    for line in lines {
        let lr = line.split(" -> ").collect::<Vec<&str>>();
        let op = lr[0].split(" ").collect::<Vec<&str>>();
        operations.insert([op[0].to_string(), op[2].to_string(), lr[1].to_string()], op[1].to_string());
    }

    Ok((initial_values, operations))
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let (initial_values, operations) = read_file("data/day24/day24.txt")?;

    let z_value = part1(&initial_values, &operations);
    println!("Part 1: {}", z_value);


    // Read in modified example (manually adjusted)
    let (initial_values, operations) = read_file("data/day24/day24-modified.txt")?;

    let z_value = part1(&initial_values, &operations);

    // Extract x and y values from target
    let x_value = extract_value(&initial_values, 'x');
    let y_value = extract_value(&initial_values, 'y');
    let expected_z = x_value + y_value;

    let bits = decimal_to_bits(z_value);
    print! {"z - file: "}
    bits.iter().for_each(|&x| if x { print!("1") } else { print!("0") });
    println!();

    print! {"z -  exp: "}
    let bits = decimal_to_bits(expected_z);
    bits.iter().for_each(|&x| if x { print!("1") } else { print!("0") });
    println!();

    let mut ans = vec!["wpd", "z11", "skh", "jqf", "z19", "mdd", "z37", "wts"];
    ans.sort();
    ans.iter().for_each(|&x| print!("{},", x));
    println!();

    Ok(())
}
