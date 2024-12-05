use std::fs::File;
use std::io::{BufRead, BufReader};

fn test_xmas(chars: &Vec<Vec<char>>, i_ref: usize, j_ref: usize) -> i32 {
    let mut count = 0;

    if i_ref >= 3 {
        
    }

    count
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

    for i in 0..chars.len() {
        for j in 0..chars[0].len() {
            if chars[i][j] == 'X' {
                count += test_xmas(&chars, i, j);
            }
        }
    }

    println!("{}", count);
}

