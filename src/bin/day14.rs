use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct State {
    pos: [i64; 2],
    vel: [i64; 2],
}

fn i64_mod(x: i64, v: i64) -> i64 {
    ((x % v) + v) % v
}

fn part1(robots: &Vec<State>, steps: i64, x_width: i64, y_width: i64) {
    let mut new_locations = Vec::new();
    for robot in robots {
        let new_x = i64_mod(robot.pos[0] + steps * robot.vel[0], x_width);
        let new_y = i64_mod(robot.pos[1] + steps * robot.vel[1], y_width);
        new_locations.push([new_x, new_y]);
    }

    let mut counts = [0_u64; 4];
    for location in new_locations {
        if location[0] < x_width / 2 && location[1] < y_width / 2 {
            counts[0] += 1;
        } else if location[0] < x_width / 2 && location[1] > y_width / 2 {
            counts[1] += 1;
        } else if location[0] > x_width / 2 && location[1] < y_width / 2 {
            counts[2] += 1;
        } else if location[0] > x_width / 2 && location[1] > y_width / 2 {
            counts[3] += 1;
        }
    }

    println!("Part 1: {:?}", counts[0] * counts[1] * counts[2] * counts[3]);
}


fn part2(robots: &Vec<State>, x_width: i64, y_width: i64) {
    let mut new_locations = HashSet::new();
    let mut steps = 0;
    loop {
        new_locations.clear();
        for robot in robots {
            let new_x = i64_mod(robot.pos[0] + steps * robot.vel[0], x_width);
            let new_y = i64_mod(robot.pos[1] + steps * robot.vel[1], y_width);
            if new_locations.contains(&[new_x, new_y]) {
                break;
            }
            new_locations.insert([new_x, new_y]);
        }

        // Check for robots in distinct locations
        if new_locations.len() == robots.len() {
            break;
        }
        steps += 1;
    }

    println!("Part 2: {:?}", steps);
}


fn get_values(s: &str) -> [i64; 2] {
    s.split(',')
        .map(|s| s.trim_start().parse().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day14/day14.txt")?;

    let mut robots = Vec::new();
    for line in file_str.lines() {
        let pv = line.split_ascii_whitespace().collect::<Vec<&str>>();
        robots.push(State {
            pos: get_values(&pv[0][2..]),
            vel: get_values(&pv[1][2..]),
        });
    }

    // part1(&robots, 100, 11, 7);
    part1(&robots, 100, 101, 103);
    part2(&robots, 101, 103);

    Ok(())
}