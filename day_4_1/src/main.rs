use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn count_xmas(matrix: &Vec<Vec<char>>) -> usize {
    let re_xmas = Regex::new(r"XMAS").unwrap();
    let re_smax = Regex::new(r"XMAS").unwrap();

    let mut total_count = 0;

    for row in matrix {
        let line: String = row.iter().collect();

        total_count += re_xmas.find_iter(&line).count();
        total_count += re_smax.find_iter(&line).count();
    }

    total_count
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let chars: Vec<Vec<char>> = reader.lines()
        .map(|line| {
            line.unwrap().chars().collect()
        })
        .collect();

    // TODO: Everything
}

