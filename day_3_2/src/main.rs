use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut mul_enabled = true;

    let outer_sum: i64 = reader.lines()
        .map(|line| {
            let line = line.unwrap();

            let mut offset = 0;
            let mut inner_sum: i64 = 0;

            loop {
                if !mul_enabled {
                    if let Some(m) = re_do.find_at(&line, offset) {
                        offset = m.end();
                        mul_enabled = true;
                    } else {
                        break;
                    }
                } else {
                    let next_dont_index = if let Some(m) = re_dont.find_at(&line, offset) {
                        m.start()
                    } else {
                        line.len()
                    };

                    if let Some(m) = re_mul.find_at(&line, offset) {
                        if m.start() < next_dont_index {
                            let (_, [x, y]) = re_mul.captures_at(&line, offset).unwrap().extract();

                            let x = x.parse::<i64>().unwrap();
                            let y = y.parse::<i64>().unwrap();

                            offset = m.end();
                            inner_sum += x * y;
                        } else {
                            mul_enabled = false;
                            offset = next_dont_index;
                        }
                    } else {
                        break;
                    }
                }
            }

            inner_sum
        })
        .sum();

    println!("{}", outer_sum);
}

