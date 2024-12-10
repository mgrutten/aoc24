use array2d::Array2D;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn find_locations(map: &Array2D<char>, frequency: char) -> Vec<(i32, i32)> {
    let mut locations = Vec::new();
    for row in 0..map.num_rows() {
        for col in 0..map.num_columns() {
            if map[(row, col)] == frequency {
                locations.push((row as i32, col as i32));
            }
        }
    }

    locations
}


fn check_location(location: &(i32, i32), map_size: &(i32, i32)) -> bool {
    location.0 < map_size.0 && location.0 >= 0 &&
        location.1 < map_size.1 && location.1 >= 0
}

fn part1(map: &Array2D<char>) {
    // Find unique frequencies
    let mut frequencies = map.elements_row_major_iter().collect::<HashSet<_>>();
    frequencies.remove(&'.');

    let map_size = (map.num_rows() as i32, map.num_columns() as i32);

    // Loop over each frequency
    let mut antinodes = HashSet::new();
    for frequency in frequencies {
        let locations = find_locations(map, *frequency);

        // Loop over each pair of locations
        for l1 in 0..locations.len() {
            for l2 in (l1 + 1)..locations.len() {
                let row_diff = locations[l1].0 - locations[l2].0;
                let col_diff = locations[l1].1 - locations[l2].1;

                // Check for possible antinode locations
                let new_l1 = (locations[l1].0 + row_diff, locations[l1].1 + col_diff);
                if check_location(&new_l1, &map_size) {
                    antinodes.insert(new_l1);
                }
                let new_l2 = (locations[l2].0 - row_diff, locations[l2].1 - col_diff);
                if check_location(&new_l2, &map_size) {
                    antinodes.insert(new_l2);
                }
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
}


fn part2(map: &Array2D<char>) {
    // Find unique frequencies
    let mut frequencies = map.elements_row_major_iter().collect::<HashSet<_>>();
    frequencies.remove(&'.');

    let map_size = (map.num_rows() as i32, map.num_columns() as i32);

    // Loop over each frequency
    let mut antinodes = HashSet::new();
    for frequency in frequencies {
        let locations = find_locations(map, *frequency);

        // Loop over each pair of locations
        for l1 in 0..locations.len() {
            for l2 in (l1 + 1)..locations.len() {
                let row_diff = locations[l1].0 - locations[l2].0;
                let col_diff = locations[l1].1 - locations[l2].1;

                // Check for possible antinode locations (forward)
                let mut scale = 0;
                loop {
                    let new_location =
                        (locations[l1].0 + scale * row_diff, locations[l1].1 + scale * col_diff);
                    if !check_location(&new_location, &map_size) {
                        break;
                    }
                    scale += 1;
                    antinodes.insert(new_location);
                }

                scale = -1;
                loop {
                    let new_location =
                        (locations[l1].0 + scale * row_diff, locations[l1].1 + scale * col_diff);
                    if !check_location(&new_location, &map_size) {
                        break;
                    }
                    scale -= 1;
                    antinodes.insert(new_location);
                }
            }
        }
    }

    println!("Part 2: {}", antinodes.len());
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day08/day08.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let map = Array2D::from_rows(&map_vec).unwrap();

    part1(&map);
    part2(&map);

    Ok(())
}