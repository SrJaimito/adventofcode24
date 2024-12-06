use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn count_xmas(matrix: &Vec<Vec<char>>) -> usize {
    let re_xmas = Regex::new(r"XMAS").unwrap();
    let re_samx = Regex::new(r"SAMX").unwrap();

    let mut total_count = 0;

    for row in matrix {
        let line: String = row.iter().collect();

        total_count += re_xmas.find_iter(&line).count();
        total_count += re_samx.find_iter(&line).count();
    }

    total_count
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut chars: Vec<Vec<char>> = reader.lines()
        .map(|line| {
            line.unwrap().chars().collect()
        })
        .collect();

    let mut total_count = 0;

    total_count += count_xmas(&chars);

    /////////////////////////////////////////////////////////////////

    let mut diag_chars: Vec<Vec<char>> = vec![];

    for i in 0..chars.len() {
        let mut row: Vec<char> = vec![];
        for j in 0..=i {
            row.push(chars[i - j][j]);
        }

        diag_chars.push(row);
    }

    for j in 1..chars.len() {
        let mut row: Vec<char> = vec![];
        for i in 0..(chars.len() - j) {
            row.push(chars[chars.len() - 1 - i][j + i]);
        }

        diag_chars.push(row);
    }

    total_count += count_xmas(&diag_chars);

    /////////////////////////////////////////////////////////////////

    let mut diag_chars: Vec<Vec<char>> = vec![];

    for j in (1..chars.len()).rev() {
        let mut row: Vec<char> = vec![];
        for i in 0..(chars.len() - j) {
            row.push(chars[i][j + i]);
        }

        diag_chars.push(row);
    }

    for i in 0..chars.len() {
        let mut row: Vec<char> = vec![];
        for j in 0..(chars.len() - i) {
            row.push(chars[i + j][j]);
        }

        diag_chars.push(row);
    }

    total_count += count_xmas(&diag_chars);

    /////////////////////////////////////////////////////////////////

    for i in 0..(chars.len() - 1) {
        for j in (i + 1)..chars.len() {
            let aux = chars[i][j];
            chars[i][j] = chars[j][i];
            chars[j][i] = aux;
        }
    }

    total_count += count_xmas(&chars);

    println!("{}", total_count);
}

