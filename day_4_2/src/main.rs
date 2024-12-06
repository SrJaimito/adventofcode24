use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_x_mas(chars: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let diag_1: String = [
        chars[i - 1][j - 1],
        chars[i][j],
        chars[i + 1][j + 1]
    ].iter().collect();

    let diag_2: String = [
        chars[i - 1][j + 1],
        chars[i][j],
        chars[i + 1][j - 1]
    ].iter().collect();

    if diag_1 == "MAS" || diag_1 == "SAM" {
        if diag_2 == "MAS" || diag_2 == "SAM" {
            return true;
        }
    }

    false
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let chars: Vec<Vec<char>> = reader.lines()
        .map(|line| {
            line.unwrap().chars().collect()
        })
        .collect();

    let mut count = 0;

    for i in 1..(chars.len() - 1) {
        for j in 1..(chars[i].len() - 1) {
            if chars[i][j] == 'A' {
                if is_x_mas(&chars, i, j) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

