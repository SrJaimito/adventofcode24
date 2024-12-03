use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn get_sum_in_scope(target: &str) -> i64 {
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re_mul.captures_iter(target)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();

            let x = x.parse::<i64>().unwrap();
            let y = y.parse::<i64>().unwrap();

            x * y
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut mul_enabled = true;

    let outer_sum: i64 = reader.lines()
        .map(|line| {
            let line = line.unwrap();

            let mut search_scope: (usize, usize) = (0, 0);
            let mut inner_sum: i64 = 0;

            loop {
                match re_do.find(&line[(search_scope.1)..]) {
                    Some(m) => {
                        search_scope.0 = m.start();
                    },
                    None => break
                }

                if let Some(m) = re_dont.find(&line[(search_scope.0)..]) {
                    search_scope.1 = m.start();
                }

                inner_sum += get_sum_in_scope(&line[(search_scope.0)..(search_scope.1)]);
            }

            inner_sum
        })
        .sum();

    println!("{}", outer_sum);
}

