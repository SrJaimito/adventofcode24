use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let num_safe = reader.lines()
        .filter(|line| {
            let signals: Vec<i32> = line.as_ref().unwrap().split_whitespace()
                .map(|word| {
                    word.parse::<i32>().unwrap()
                })
                .collect();

            let increasing = signals[0] < signals[1];
            let mut is_safe = true;

            for i in 0..(signals.len() - 1) {
                if increasing {
                    if (signals[i + 1] > signals[i] + 3) || (signals[i + 1] < signals[i] + 1) {
                        is_safe = false;
                        break;
                    }
                } else {
                    if (signals[i + 1] < signals[i] - 3) || (signals[i + 1] > signals[i] - 1) {
                        is_safe = false;
                        break;
                    }
                }
            }

            is_safe
        })
        .count();

    println!("{}", num_safe);
}

