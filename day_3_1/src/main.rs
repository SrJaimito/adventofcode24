use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let outer_sum: i64 = reader.lines()
        .map(|line| {
            let line = line.unwrap();

            let inner_sum: i64 = re.captures_iter(&line)
                .map(|caps| {
                    let (_, [x, y]) = caps.extract();

                    let x = x.parse::<i64>().unwrap();
                    let y = y.parse::<i64>().unwrap();

                    x * y
                })
                .sum();

            inner_sum
        })
        .sum();

    println!("{}", outer_sum);
}

