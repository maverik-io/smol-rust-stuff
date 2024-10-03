/*
 * Author: Joseph Chacko;
 */

use std::io::Write;
type Matrix = Vec<Vec<f64>>;
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

fn parse(inp: String) -> Vec<f64> {
    let mut out = vec![];
    for i in inp.split(' ') {
        match i.parse::<f64>() {
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

fn row_echelon_form(matrix: &mut Vec<Vec<f64>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut i = 0;
    let mut j = 0;

    while i < m && j < n {
        // Find the first nonzero element in the current column
        let mut pivot_row = i;
        while pivot_row < m && matrix[pivot_row][j] == 0.0 {
            pivot_row += 1;
        }

        // If no nonzero element is found, move to the next column
        if pivot_row == m {
            j += 1;
            continue;
        }

        // Swap the current row with the row containing the pivot
        if pivot_row != i {
            matrix.swap(i, pivot_row);
        }

        // Divide the current row by the pivot
        let pivot = matrix[i][j];
        for k in j..n {
            matrix[i][k] /= pivot;
        }

        // Eliminate entries below the pivot
        for k in i + 1..m {
            let factor = matrix[k][j];
            for l in j..n {
                matrix[k][l] -= factor * matrix[i][l];
            }
        }

        i += 1;
        j += 1;
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
    row_echelon_form(&mut matrix);
    pprint(&matrix, Some(vec![String::from("Answer")]));
    println!("{}=", "=+".repeat(21));
}
