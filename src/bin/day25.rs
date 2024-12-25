use array2d::Array2D;
use std::error::Error;
use std::fs;

fn part1(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) {
    let mut fits = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(lv, kv)| lv + kv <= 7) {
                fits += 1;
            }
        }
    }
    println!("fits: {}", fits);
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day25/day25.txt")?;

    let mut schematics = Vec::new();
    let mut lines = file_str.lines();
    let mut schematic_vec = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            schematics.push(Array2D::from_rows(&schematic_vec)?);
            schematic_vec = Vec::new();
            continue;
        }
        let row = line.chars()
            .map(|c| c == '#')
            .collect::<Vec<_>>();
        schematic_vec.push(row);
    }
    schematics.push(Array2D::from_rows(&schematic_vec)?);

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in schematics {
        let col_count = schematic.columns_iter()
            .map(|itr| itr.filter(|&v| *v).count())
            .collect::<Vec<_>>();
        if schematic.row_iter(0)?.all(|&c| c) {
            locks.push(col_count);
        } else {
            keys.push(col_count);
        }
    }

    part1(&locks, &keys);

    Ok(())
}
