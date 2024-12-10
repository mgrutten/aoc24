use array2d::Array2D;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn increment(location: (i32, i32), direction: (i32, i32), size: (i32, i32)) -> Option<(i32, i32)> {
    let new_location = (location.0 + direction.0, location.1 + direction.1);
    if new_location.0 < 0 || new_location.1 < 0 ||
        new_location.0 >= size.0 || new_location.1 >= size.1 {
        None
    } else {
        Some(new_location)
    }
}

fn count_paths(map: &Array2D<u32>, location: (i32, i32), height: u32) -> HashSet<(i32, i32)> {
    if height == 9 {
        let mut paths = HashSet::new();
        paths.insert(location);
        return paths;
    }

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let map_size = (map.num_rows() as i32, map.num_columns() as i32);

    let mut paths = HashSet::new();
    for direction in directions {
        if let Some(new_location) = increment(location, direction, map_size) {
            if map[(new_location.0 as usize, new_location.1 as usize)] == height + 1 {
                paths.extend(&count_paths(&map, new_location, height + 1));
            }
        }
    }

    paths
}


fn count_all_paths(map: &Array2D<u32>, location: (i32, i32), height: u32) -> i32 {
    if height == 9 {
        return 1;
    }

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let map_size = (map.num_rows() as i32, map.num_columns() as i32);

    let mut paths = 0;
    for direction in directions {
        if let Some(new_location) = increment(location, direction, map_size) {
            if map[(new_location.0 as usize, new_location.1 as usize)] == height + 1 {
                paths += count_all_paths(&map, new_location, height + 1);
            }
        }
    }

    paths
}


fn find_trailheads(map: &Array2D<u32>) -> HashSet<(i32, i32)> {
    let mut trailheads = HashSet::new();
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            if map[(row, col)] == 0 {
                trailheads.insert((row as i32, col as i32));
            }
        }
    }

    trailheads
}


fn part1(map: &Array2D<u32>) {
    let path_total = find_trailheads(map).iter()
        .fold(0, |acc, trailhead| acc + count_paths(map, *trailhead, 0).len());

    println!("Part 1: {}", path_total);
}

fn part2(map: &Array2D<u32>) {
    let path_total = find_trailheads(map).iter()
        .fold(0, |acc, trailhead| acc + count_all_paths(map, *trailhead, 0));

    println!("Part 2: {}", path_total);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day10/day10.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars().map(|c| c as u32 - '0' as u32).collect())
        .collect::<Vec<Vec<u32>>>();
    let map = Array2D::from_rows(&map_vec).unwrap();

    part1(&map);
    part2(&map);

    Ok(())
}