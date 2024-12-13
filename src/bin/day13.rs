use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Problem {
    button_a: [i64; 2],
    button_b: [i64; 2],
    prize: [i64; 2],
}


fn count_tokens(problems: &Vec<Problem>, factor: i64) {
    let mut cost = 0;
    for problem in problems {

        // Calculate determinant (ax * by - ay * bx)
        let determinant = problem.button_a[0] * problem.button_b[1] -
            problem.button_a[1] * problem.button_b[0];

        // Calculate partial solution
        // (by * x - bx * y)
        let a_factor = problem.button_b[1] * (factor + problem.prize[0]) -
            problem.button_b[0] * (factor + problem.prize[1]);
        // (-ay * x + ax * y)
        let b_factor = -problem.button_a[1] * (factor + problem.prize[0]) +
            problem.button_a[0] * (factor + problem.prize[1]);

        // Check partial solution is (integer) divisible by the determinant
        if (a_factor % determinant == 0) && (b_factor % determinant == 0) {
            cost += 3 * a_factor / determinant + b_factor / determinant;
        }
    }

    println!("Cost: {}", cost);
}


fn get_values(s: &str) -> [i64; 2] {
    let lr = s.split(',').map(|s| s.trim_start()).collect::<Vec<&str>>();
    [lr[0][2..].parse::<i64>().unwrap(), lr[1][2..].parse::<i64>().unwrap()]
}

fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day13/day13.txt")?;

    let mut problems = Vec::new();
    for chunk in file_str.lines().collect::<Vec<&str>>().iter().as_slice().chunks(4) {
        problems.push(Problem {
            button_a: get_values(&chunk[0][10..]),
            button_b: get_values(&chunk[1][10..]),
            prize: get_values(&chunk[2][7..]),
        });
    }

    count_tokens(&problems, 0);
    count_tokens(&problems, 10000000000000);

    Ok(())
}