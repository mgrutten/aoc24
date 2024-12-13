use array2d::Array2D;
use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
struct Problem {
    buttons: Array2D<i64>,
    prize: Array2D<i64>,
}


fn count_tokens(problems: &Vec<Problem>, factor: i64) {
    let mut cost = 0;
    for problem in problems {
        // For matrix
        // |ax bx|
        // |ay by|
        
        // Calculate determinant (ax * by - bx * ay)
        let determinant = problem.buttons[(0, 0)] * problem.buttons[(1, 1)] -
            problem.buttons[(0, 1)] * problem.buttons[(1, 0)];

        // Calculate partial solution
        // (by * x - bx * y)
        let a_factor = problem.buttons[(1, 1)] * (factor + problem.prize[(0, 0)]) -
            problem.buttons[(0, 1)] * (factor + problem.prize[(1, 0)]);
        // (-ay * x + ax * y)
        let b_factor = -problem.buttons[(1, 0)] * (factor + problem.prize[(0, 0)]) +
            problem.buttons[(0, 0)] * (factor + problem.prize[(1, 0)]);

        // Check partial solution is (integer) divisible by the determinant
        if (a_factor % determinant == 0) && (b_factor % determinant == 0) {
            cost += 3 * a_factor / determinant + b_factor / determinant;
        }
    }

    println!("Cost: {}", cost);
}


fn main() -> Result<(), Box<dyn Error>> {

    // Read in example
    let file_str: String = fs::read_to_string("data/day13/day13.txt")?;

    let mut problems = Vec::new();
    for chunk in file_str.lines().collect::<Vec<&str>>().iter().as_slice().chunks(4) {
        let lr = chunk[0][10..].split(',').map(|s| s.trim_start()).collect::<Vec<&str>>();
        let ax = lr[0][2..].parse::<i64>().unwrap();
        let ay = lr[1][2..].parse::<i64>().unwrap();

        let lr = chunk[1][10..].split(',').map(|s| s.trim_start()).collect::<Vec<&str>>();
        let bx = lr[0][2..].parse::<i64>().unwrap();
        let by = lr[1][2..].parse::<i64>().unwrap();

        let lr = chunk[2][7..].split(',').map(|s| s.trim_start()).collect::<Vec<&str>>();
        let px = lr[0][2..].parse::<i64>().unwrap();
        let py = lr[1][2..].parse::<i64>().unwrap();

        problems.push(Problem {
            buttons: Array2D::from_row_major(&[ax, bx, ay, by], 2, 2).unwrap(),
            prize: Array2D::from_row_major(&[px, py], 2, 1).unwrap(),
        });
    }

    count_tokens(&problems, 0);
    count_tokens(&problems, 10000000000000);

    Ok(())
}