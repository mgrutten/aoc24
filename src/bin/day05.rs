use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;


pub fn check_rule(rules: &Vec<i32>, pages: &HashSet<&i32>) -> bool {
    for rule in rules {
        if pages.contains(&rule) {
            return false;
        }
    }

    true
}

pub fn is_correct_order(rules: &HashMap<i32, Vec<i32>>, page_order: &Vec<i32>) -> bool {
    for i in 0..page_order.len() {
        if rules.contains_key(&page_order[i]) {

            // If the page is in the rules, then check remaining pages against the rule
            let remaining_pages = page_order[i + 1..].into_iter()
                .collect::<HashSet<_>>();

            if !remaining_pages.is_empty() &&
                !check_rule(rules.get(&page_order[i]).unwrap(), &remaining_pages) {
                return false;
            }
        }
    }

    true
}

pub fn part1(rules: &HashMap<i32, Vec<i32>>, page_orders: &Vec<Vec<i32>>) {
    let mut middle_sum = 0;
    for page_order in page_orders {
        if is_correct_order(&rules, page_order) {
            // Find middle page number
            middle_sum += page_order[(page_order.len() - 1) / 2];
        }
    }

    println!("Part 1: {}", middle_sum);
}


pub fn shuffle(rules: &HashMap<i32, Vec<i32>>, page_order: &Vec<i32>) -> Vec<i32> {
    let mut new_order = page_order.clone();

    for i in 0..new_order.len() {
        if rules.contains_key(&new_order[i]) {

            // The rule to check
            let rule = rules.get(&new_order[i]).unwrap().into_iter()
                .collect::<HashSet<&i32>>();

            // If the page is in the rules, then check remaining pages against the rule
            for j in (i + 1)..new_order.len() {
                // If there's a violating rule, swap the order
                if rule.contains(&new_order[j]) {
                    new_order.swap(i, j);
                    break;
                }
            }
        }
    }

    new_order
}

pub fn part2(rules: &HashMap<i32, Vec<i32>>, page_orders: &Vec<Vec<i32>>) {
    let mut middle_sum = 0;
    for page_order in page_orders {
        if !is_correct_order(&rules, page_order) {
            // If it's not in the correct order, keep shuffling until it is
            let mut new_order = shuffle(rules, page_order);
            while !is_correct_order(&rules, &new_order) {
                new_order = shuffle(rules, &new_order);
            }

            middle_sum += new_order[(page_order.len() - 1) / 2];
        }
    }

    println!("Part 2: {}", middle_sum);
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day05/day05.txt")?;

    let mut rules = HashMap::new();
    let mut page_orders = Vec::new();

    // Parse into rules and page orders
    // Each rule contains a vec of pages that must come -before-
    let mut is_rules = true;
    for line in file_str.lines() {
        if line.is_empty() {
            is_rules = false;
        } else {
            if is_rules {
                let before = line[0..2].parse::<i32>()?;
                let after = line[3..].parse::<i32>()?;

                rules.entry(after).or_insert(Vec::new()).push(before);
            } else {
                let order = line.split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                page_orders.push(order);
            }
        }
    }

    part1(&rules, &page_orders);
    part2(&rules, &page_orders);

    Ok(())
}