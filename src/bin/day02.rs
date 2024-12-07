use std::error::Error;
use std::fs;


pub fn is_report_safe(report: &Vec<i32>) -> bool {
    let is_increasing = *report.last().unwrap() > report[0];

    let min_increase = report.windows(2)
        .map(|window| window[1] - window[0])
        .min()
        .unwrap();

    let max_increase = report.windows(2)
        .map(|window| window[1] - window[0])
        .max()
        .unwrap();

    if is_increasing {
        min_increase >= 1 && max_increase <= 3
    } else {
        min_increase >= -3 && max_increase <= -1
    }
}

pub fn is_report_safe_part2(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        true
    } else {

        // Remove each element in turn and test for being safe
        for i in 0..report.len() {
            let mut report_rem = report.clone();
            report_rem.remove(i);

            if is_report_safe(&report_rem) {
                return true;
            }
        }

        false
    }
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day02/day02.txt")?;

    // Parse into reports
    let mut reports = Vec::new();
    for line in file_str.lines() {
        let report = line.split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        reports.push(report);
    }

    // Part 1
    let safe_count = reports.iter()
        .fold(0, |acc, report| if is_report_safe(report) { acc + 1 } else { acc });
    println!("Part 1: {}", safe_count);

    // Part 2
    let safe_count = reports.iter()
        .fold(0, |acc, report| if is_report_safe_part2(report) { acc + 1 } else { acc });
    println!("Part 2: {}", safe_count);

    Ok(())
}