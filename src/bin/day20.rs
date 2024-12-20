use array2d::Array2D;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum MapContent {
    Wall,
    Empty,
    Start,
    End,
}

/*
impl fmt::Display for MapContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapContent::Wall => write!(f, "#"),
            MapContent::Start => write!(f, "S"),
            MapContent::End => write!(f, "E"),
            MapContent::Empty => write!(f, "."),
            MapContent::Visited => write!(f, "O"),
        }
    }
}


fn print_map(map: &Array2D<MapContent>) {
    for row in map.rows_iter() {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
}

 */


fn find_start(map: &Array2D<MapContent>) -> (usize, usize) {
    map.enumerate_row_major()
        .find(|(_, val)| **val == MapContent::Start)
        .unwrap().0
}


fn initial_path(map: &Array2D<MapContent>) -> Vec<(usize, usize)> {
    // Single possible route to end
    let mut route = Vec::new();
    let mut location = find_start(map);
    let mut previous = location;
    route.push(location);

    'outer: while map[location] != MapContent::End {
        for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new_row = location.0 as i32 + direction.0;
            let new_col = location.1 as i32 + direction.1;

            let new_location = (new_row as usize, new_col as usize);
            if new_location == previous {
                continue;
            }

            if map[new_location] != MapContent::Wall {
                previous = location;
                location = new_location;
                route.push(location);
                continue 'outer;
            }
        }
    }

    route
}


fn find_shortcuts(path: &Vec<(usize, usize)>, threshold: i32, shortcut: i32) {
    let mut count: usize = 0;
    for idx1 in 0..path.len() {
        for idx2 in idx1 + 1..path.len() {
            let l1 = path[idx1];
            let l2 = path[idx2];
            let diff = (l1.0 as i32 - l2.0 as i32).abs() + (l1.1 as i32 - l2.1 as i32).abs();

            if diff <= shortcut {
                let dist = (idx2 - idx1) as i32 - diff;
                if dist >= threshold {
                    count += 1;
                }
            }
        }
    }

    println!("count: {}", count);
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day20/day20.txt")?;

    let map_vec = file_str.lines()
        .map(|line| line.chars()
            .map(|c| match c {
                '#' => MapContent::Wall,
                'S' => MapContent::Start,
                'E' => MapContent::End,
                '.' => MapContent::Empty,
                _ => unreachable!()
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = Array2D::from_rows(&map_vec).unwrap();

    let path = initial_path(&map);

    find_shortcuts(&path, 100, 2);
    find_shortcuts(&path, 100, 20);

    Ok(())
}