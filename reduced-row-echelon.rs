/*
 * Author: Joseph Chacko;
 * Notes : Uses Jidu's RREF algorithm to find the RREF of a 3x3 matrix;
 */

use std::io::Write;
type Matrix = Vec<Vec<i32>>;
type List = Vec<String>;

fn input(prompt: String) -> String {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    let mut out = String::new();
    std::io::stdin().read_line(&mut out).unwrap();
    print!("\x1B[F\x1B[K");
    std::io::stdout().flush().unwrap();
    out.trim().to_string()
}

fn parse(inp: String) -> Vec<i32> {
    let mut out = vec![];
    for i in inp.split(' ') {
        match i.parse::<i32>() {
            Ok(val) => out.push(val),
            _ => (),
        }
    }
    out
}

fn pprint(matrix: &Matrix, maybe_steps: Option<List>) {
    let mut steps: List = vec![];
    if maybe_steps.is_some() {
        steps = maybe_steps.unwrap().into_iter().rev().collect();
    }

    let mut lengths = vec![0, 0, 0];
    println!("{}=", "=+".repeat(21));
    for row in matrix {
        for i in 0..3 {
            if row[i].to_string().len() > lengths[i] {
                lengths[i] = row[i].to_string().len();
            }
        }
    }

    let padding = 32 - lengths.clone().into_iter().fold(0, |t, v| t + v);
    for row in matrix {
        print!("| ");
        let mut i = 0;
        for elem in row {
            print!("{elem:>a$} ", a = lengths[i]);
            i += 1;
        }
        println!(
            "|     {:>padding$}",
            steps.pop().unwrap_or(String::from(""))
        );
    }
}

fn lcmof(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    let primes = [2, 3, 5, 7, 11, 13, 17];
    let mut found = false;
    let mut factors = vec![];
    for p in primes {
        let mut used = true;
        while used == true {
            used = false;
            if a % p == 0 {
                a = a / p;
                used = true;
            }
            if b % p == 0 {
                b = b / p;
                used = true;
            }
            if c % p == 0 {
                c = c / p;
                used = true;
            }
            if used {
                factors.push(p)
            }
        }
        if a == 1 && b == 1 && c == 1 {
            found = true;
            break;
        }
    }
    if found {
        factors.into_iter().fold(1, |t, v| t * v)
    } else {
        a * b * c
    }
}
fn is_solved(matrix: &Matrix) -> bool {
    let (a, b, c) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    if a == 1 && b == 0 && c == 0 {
        if matrix[1][1] == 1 {
            if matrix[2][1] == 0 {
                if matrix[2][2] <= 1 {
                    return true;
                }
            }
        } else if matrix[1][1] == 0 {
            if matrix[1][2] == 0 && matrix[2][1] == 0 && matrix[2][2] == 0 {
                return true;
            }
        }
    }
    false
}
fn solve(mut matrix: Matrix) {
    // Step 1 - LCM Magic V1
    let step1_lcm = lcmof(matrix[0][0], matrix[1][0], matrix[2][0]);
    let mut step1_multipliers = vec![];
    for i in 0..3 {
        let multiplier = step1_lcm / matrix[i][0];
        step1_multipliers.push(multiplier);
        for j in 0..3 {
            matrix[i][j] = matrix[i][j] * multiplier;
        }
    }
    let steps = {
        let mut i = 1;
        let mut out = vec![];
        for n in step1_multipliers {
            out.push(format!("R{i} => R{i} × {n}"));
            i += 1;
        }
        out
    };
    pprint(&matrix, Some(steps));

    // Step 2 - Subtract V1
    for i in 1..3 {
        for j in 0..3 {
            matrix[i][j] -= matrix[0][j];
        }
    }
    let steps = {
        let mut out = vec!["".to_string()];
        for i in 2..4 {
            out.push(format!("R{i} => R{i} - R1"));
        }
        out
    };

    pprint(&matrix, Some(steps));

    // Check if solved already - there is a possibility?
    if is_solved(&matrix) {
        pprint(&matrix, Some(vec![String::from("Answer")]));
        return;
    }
    if [matrix[1][1], matrix[1][2], matrix[2][2], matrix[2][1]]
        .iter()
        .fold(0, |t, v| t + v)
        != 0
    {
        // Step 3 - LCM Magic V2
        let step2_lcm = lcmof(1, matrix[1][1], matrix[2][1]);
        let mut step2_multipliers = vec![];
        for i in 1..3 {
            let multiplier = step2_lcm / matrix[i][1];
            step2_multipliers.push(multiplier);
            for j in 0..3 {
                matrix[i][j] = matrix[i][j] * multiplier;
            }
        }
        let steps = {
            let mut i = 1;
            let mut out = vec!["".to_string()];
            for n in step2_multipliers {
                out.push(format!("R{i} => R{i} × {n}"));
                i += 1;
            }
            out
        };
        pprint(&matrix, Some(steps));

        for j in 0..3 {
            matrix[2][j] -= matrix[1][j];
        }

        // Step 4 - Subtract V2
        let steps = vec![format!("R3 => R3 - R2")];
        pprint(&matrix, Some(steps));

        // Check if solved already - there is a possibility?
        if is_solved(&matrix) {
            pprint(&matrix, Some(vec![String::from("Answer")]));
            return;
        }
    }
    // Step 5 - Divide every row to start with 1
    let step5_divisors = vec![matrix[0][0], matrix[1][1], matrix[2][2]];

    let e0 = matrix[0][0];
    for i in 0..3 {
        matrix[0][i] /= e0
    }
    let e1 = matrix[1][1];
    if matrix[1][1] != 0 {
        for i in 1..3 {
            matrix[1][i] /= e1
        }
    }
    matrix[2][2] = {
        if matrix[2][2] == 0 {
            0
        } else {
            1
        }
    };

    let steps = {
        let mut i = 1;
        let mut out = vec![];
        for n in step5_divisors {
            out.push({
                if n == 0 {
                    "".to_string()
                } else {
                    format!("R{i} => R{i} ÷ {n}")
                }
            });
            i += 1;
        }
        out
    };
    pprint(&matrix, Some(steps));

    // Check if solved already - there is a possibility?
    if is_solved(&matrix) {
        pprint(&matrix, Some(vec![String::from("Answer")]));
        return;
    }
}

fn main() {
    let mut matrix = vec![];
    for i in 1..4 {
        let prompt = format!("Row {i}: ");
        let inp = input(prompt);
        matrix.push(parse(inp));
    }
    pprint(&matrix, Some(vec![String::from("Question")]));
    solve(matrix);
    println!("{}=", "=+".repeat(21));
}
