use array2d::Array2D;
use std::cmp::PartialEq;
use std::error::Error;
use std::{fmt, fs};

#[derive(Debug, Clone, PartialEq)]
enum MapContent {
    Wall,
    Box,
    Robot,
    Empty,
}

impl fmt::Display for MapContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapContent::Wall => write!(f, "#"),
            MapContent::Box => write!(f, "O"),
            MapContent::Robot => write!(f, "@"),
            MapContent::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}


fn print_map(map: &Array2D<MapContent>) {
    for row in map.rows_iter() {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
}


fn find_coords(map: &Array2D<MapContent>, content: MapContent) -> Vec<(usize, usize)> {
    map.enumerate_row_major()
        .filter(|(_, val)| **val == content)
        .map(|(coord, _)| coord)
        .collect::<Vec<(usize, usize)>>()
}

fn add_coords(a: (usize, usize), b: (i32, i32)) -> (usize, usize) {
    ((a.0 as i32 + b.0) as usize, (a.1 as i32 + b.1) as usize)
}

fn move_robot(map: &mut Array2D<MapContent>,
              coord: (usize, usize),
              direction: &Direction) -> (usize, usize) {
    let dirn = match direction {
        Direction::Left => (0_i32, -1_i32),
        Direction::Right => (0, 1),
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
    };

    let new_coord = add_coords(coord, dirn);
    if map[new_coord] == MapContent::Empty {
        map[coord] = MapContent::Empty;
        map[new_coord] = MapContent::Robot;
        return new_coord;
    } else if map[new_coord] == MapContent::Box {
        // Look for a gap
        let mut gap_coord = add_coords(new_coord, dirn);
        while map[gap_coord] == MapContent::Box {
            gap_coord = add_coords(gap_coord, dirn);
        }
        if map[gap_coord] == MapContent::Empty {
            map[gap_coord] = MapContent::Box;
            map[coord] = MapContent::Empty;
            map[new_coord] = MapContent::Robot;
            return new_coord;
        }
    }

    coord
}


fn part1(map: &Array2D<MapContent>, moves: &Vec<Direction>) {
    let mut mut_map = map.clone();
    let mut coord = find_coords(map, MapContent::Robot)[0];

    for m in moves {
        coord = move_robot(&mut mut_map, coord, m);
    }

    let score = mut_map.enumerate_row_major()
        .filter(|(_, val)| **val == MapContent::Box)
        .map(|(coord, _)| 100 * coord.0 + coord.1)
        .sum::<usize>();

    println!("Part 1: {}", score);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day15/day15.txt")?;

    let lines = file_str.lines().collect::<Vec<&str>>();
    let mut idx = 0;

    let mut map_vec = Vec::new();
    while lines[idx].len() > 0 {
        let map_line = lines[idx].chars()
            .map(|c| match c {
                '#' => MapContent::Wall,
                'O' => MapContent::Box,
                '@' => MapContent::Robot,
                '.' => MapContent::Empty,
                _ => unreachable!()
            })
            .collect::<Vec<_>>();
        map_vec.push(map_line);

        idx += 1;
    }
    let map = Array2D::from_rows(&map_vec).unwrap();

    let mut moves = Vec::new();
    while idx < lines.len() {
        let line_moves = lines[idx].chars()
            .map(|c| match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => unreachable!()
            })
            .collect::<Vec<_>>();

        moves.extend(line_moves);
        idx += 1;
    }

    //print_map(&map);
    //println!("{:?}", moves);

    part1(&map, &moves);

    Ok(())
}