use array2d::Array2D;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fs;


#[derive(Debug, Clone, Eq, PartialEq)]
struct CostLocation {
    cost: u64,
    location: (usize, usize),
}

impl Ord for CostLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse order for min-heap
    }
}

impl PartialOrd for CostLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u64 {
    let dx = (a.0 as i32 - b.0 as i32).abs() as u64;
    let dy = (a.1 as i32 - b.1 as i32).abs() as u64;
    dx + dy
}

fn explore(map: &Array2D<u8>,
           start_location: (usize, usize),
           end_location: (usize, usize)) -> u64 {
    let moves = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    distances.insert(start_location, 0);
    heap.push(CostLocation {
        cost: manhattan_distance(start_location, end_location),
        location: start_location,
    });

    while let Some(CostLocation { cost: _, location: current_location }) = heap.pop() {
        if current_location == end_location {
            return distances[&current_location];
        }

        for m in moves.iter() {
            let new_row = current_location.0 as i32 + m.0;
            let new_col = current_location.1 as i32 + m.1;

            if new_row < 0 || new_col < 0 ||
                new_row >= map.num_rows() as i32 || new_col >= map.num_columns() as i32 {
                continue;
            }

            let new_location = (new_row as usize, new_col as usize);
            if map[new_location] == 1 {
                continue;
            }

            let new_cost = distances[&current_location] + 1;
            if new_cost < *distances.get(&new_location).unwrap_or(&u64::MAX) {
                distances.insert(new_location, new_cost);
                heap.push(CostLocation {
                    cost: new_cost + manhattan_distance(new_location, end_location),
                    location: new_location,
                });
            }
        }
    }

    u64::MAX
}


fn part1(map: &Array2D<u8>) {
    let start = (0, 0);
    let end = (map.num_rows() - 1, map.num_columns() - 1);

    let min_cost = explore(map, start, end);
    println!("Part 1: {}", min_cost);
}


fn print_map(map: &Array2D<u8>) {
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            print!("{}", map[(row, col)]);
        }
        println!();
    }
}


fn part2(coords: &Vec<(usize, usize)>) {
    let max_row = coords.iter().map(|r| r.0).max().unwrap();
    let max_col = coords.iter().map(|r| r.1).max().unwrap();

    let start = (0, 0);
    let end = (max_row, max_col);

    let mut map = Array2D::filled_by_row_major(|| 0, max_row + 1, max_col + 1);
    coords.iter().take(1024).for_each(|c| map[*c] = 1);

    for (index, coord) in coords.iter().enumerate().skip(1024) {
        map[*coord] = 1;
        let min_cost = explore(&map, start, end);

        if min_cost as usize > max_row * max_col {
            println!("Part 2: {}, {}, {:?}", index, min_cost, coord);
            break;
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day18/day18.txt")?;

    let coords = file_str.lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|v| (v[1].parse::<usize>().unwrap(), v[0].parse::<usize>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let max_row = coords.iter().map(|r| r.0).max().unwrap();
    let max_col = coords.iter().map(|r| r.1).max().unwrap();

    let mut map = Array2D::filled_by_row_major(|| 0, max_row + 1, max_col + 1);
    coords.iter().take(1024).for_each(|c| map[*c] = 1);

    // print_map(&map);
    part1(&map);
    part2(&coords);

    Ok(())
}