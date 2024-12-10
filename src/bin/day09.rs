use std::error::Error;
use std::fs;

fn part1(disk: &Vec<u32>) {
    let disk_len = disk.iter().sum::<u32>();

    // Decode disk
    let mut decoded: Vec<Option<u32>> = vec![None; disk_len as usize];

    let mut index = 0;
    for (disk_index, val) in disk.iter().enumerate() {
        if disk_index % 2 == 0 {
            for i in 0..*val {
                decoded[(index + i) as usize] = Some(disk_index as u32 / 2);
            }
        }
        index += val;
    }

    // Calculate the checksum
    let mut fwd_index = 0;
    let mut bwd_index = decoded.len() - 1;
    let mut checksum: u64 = 0;
    loop {
        // Move forward to next empty index
        while fwd_index < bwd_index && decoded[fwd_index].is_some() {
            checksum += fwd_index as u64 * decoded[fwd_index].unwrap() as u64;
            fwd_index += 1;
        }

        // Move backward to next non-empty index
        while decoded[bwd_index].is_none() {
            bwd_index -= 1;
        }

        // Check for end of calculation
        if fwd_index > bwd_index {
            break;
        }

        // Empty index checksum contribution
        checksum += fwd_index as u64 * decoded[bwd_index].unwrap() as u64;
        fwd_index += 1;
        bwd_index -= 1;
    }

    println!("Part 1: {}", checksum);
}


fn part2(disk: &Vec<u32>) {

    // Decode disk into blocks (size, value)
    let mut blocks = Vec::new();

    for (disk_index, val) in disk.iter().enumerate() {
        if disk_index % 2 == 0 {
            blocks.push((*val, Some(disk_index as u32 / 2)));
        } else {
            blocks.push((*val, None));
        }
    }

    // Go backward through blocks
    let mut bwd_index = blocks.len() - 1;
    while bwd_index > 0 {
        while blocks[bwd_index].1.is_none() {
            bwd_index -= 1;
        }

        // Look (forward) through blocks to find a swap candidate
        let mut empty_index = 0;
        while empty_index < bwd_index && (blocks[empty_index].1.is_some() || blocks[empty_index].0 < blocks[bwd_index].0) {
            empty_index += 1;
        }

        // No possible swap
        if empty_index == bwd_index {
            bwd_index -= 1;
            continue;
        }

        // Partial or full swap
        if blocks[bwd_index].0 == blocks[empty_index].0 {
            blocks.swap(empty_index, bwd_index);
        } else {
            let new_size = blocks[empty_index].0 - blocks[bwd_index].0;
            blocks.insert(empty_index, blocks[bwd_index]);
            blocks[bwd_index + 1].1 = None;
            blocks[empty_index + 1].0 = new_size;
        }
    }

    // Form checksum from blocks
    let mut index = 0;
    let mut checksum: u64 = 0;
    for block in blocks.iter() {
        if block.1.is_some() {
            for i in 0..block.0 {
                checksum += (index + i) as u64 * block.1.unwrap() as u64;
            }
        }
        index += block.0;
    }

    println!("Part 2: {}", checksum);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day09/day09.txt")?;
    let disk = file_str.chars()
        .map(|c| c as u32 - '0' as u32)
        .collect::<Vec<u32>>();

    part1(&disk);
    part2(&disk);

    Ok(())
}