use std::error::Error;
use std::fs;


fn part1(disk: &Vec<u32>) {
    let disk_len = disk.iter().sum::<u32>();
    //println!("disk_len: {}, {}", disk.len(), disk_len);

    // Decode disk
    let mut decoded: Vec<Option<u32>> = vec![None; disk_len as usize];

    let mut index = 0;
    for (disk_index, val) in disk.iter().enumerate() {
        //println!("index: {}, val: {}", disk_index, val);
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


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day09/day09-example.txt")?;
    let disk = file_str.chars()
        .map(|c| c as u32 - '0' as u32)
        .collect::<Vec<u32>>();

    part1(&disk);

    Ok(())
}