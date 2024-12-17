use std::error::Error;
use std::fs;


fn combo(operand: u8,
         register_a: u64,
         register_b: u64,
         register_c: u64) -> u64 {
    match operand {
        0..4 => operand as u64,
        4 => register_a,
        5 => register_b,
        6 => register_c,
        _ => unreachable!(),
    }
}


fn part1(register_a: u64,
         register_b: u64,
         register_c: u64,
         instructions: &Vec<[u8; 2]>) {
    let mut reg_a = register_a;
    let mut reg_b = register_b;
    let mut reg_c = register_c;

    let mut pointer = 0;
    let mut output = Vec::new();

    loop {
        let instruction = instructions[pointer];
        let opcode = instruction[0];
        let operand = instruction[1];
        let combo_operand = combo(operand, reg_a, reg_b, reg_c);
        let mut increment = 1;

        match opcode {
            // adv
            0 => reg_a = reg_a / 2_u64.pow(combo_operand as u32),
            // bxl
            1 => reg_b = reg_b ^ operand as u64,
            // bst
            2 => reg_b = combo_operand % 8,
            // jnz
            3 => if reg_a > 0 {
                pointer = operand as usize;
                increment = 0;
            },
            // bxc
            4 => reg_b = reg_b ^ reg_c,
            // out
            5 => {
                output.push((combo_operand % 8).to_string());
            }
            // bdv
            6 => reg_b = reg_a / 2_u64.pow(combo_operand as u32),
            // cdv
            7 => reg_c = reg_a / 2_u64.pow(combo_operand as u32),
            _ => unreachable!()
        }

        pointer += increment;
        if pointer >= instructions.len() {
            break;
        }
        
    }

    println!("{}", output.join(","));
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day17/day17.txt")?;

    let mut lines = file_str.lines();
    let register_a = lines.next().unwrap()[12..].parse::<u64>()?;
    let register_b = lines.next().unwrap()[12..].parse::<u64>()?;
    let register_c = lines.next().unwrap()[12..].parse::<u64>()?;

    lines.next();
    let instructions = lines.next().unwrap()[9..].bytes().enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, b)| b - b'0')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| [c[0], c[1]])
        .collect::<Vec<_>>();

    part1(register_a, register_b, register_c, &instructions);

    Ok(())
}