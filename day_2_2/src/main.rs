use std::fs::File;
use std::io::{BufReader, BufRead};

fn is_safe_pair(a: i32, b: i32, inc: bool) -> bool {
    if inc {
        if (b > a + 3) || (b < a + 1) {
            return false;
        }
    } else {
        if (b < a - 3) || (b > a - 1) {
            return false;
        }
    }

    true
}

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

            let mut increasing_count = 0;
            let mut decreasing_count = 0;

            for i in 0..(signals.len() - 1) {
                if signals[i + 1] > signals[i] {
                    increasing_count += 1;
                } else if signals[i + 1] < signals[i] {
                    decreasing_count += 1;
                }
            }

            let increasing = increasing_count > decreasing_count;

            let mut is_safe = true;

            for i in 0..(signals.len() - 1) {
                if !is_safe_pair(signals[i], signals[i + 1], increasing) {
                    let mut is_safe_after_remove_left = true;
                    let mut signals_remove_left = signals.clone();
                    signals_remove_left.remove(i);

                    for j in 0..(signals_remove_left.len() - 1) {
                        if !is_safe_pair(signals_remove_left[j], signals_remove_left[j + 1], increasing) {
                            is_safe_after_remove_left = false;
                            break;
                        }
                    }

                    let mut is_safe_after_remove_right = true;
                    let mut signals_remove_right = signals.clone();
                    signals_remove_right.remove(i + 1);

                    for j in 0..(signals_remove_right.len() - 1) {
                        if !is_safe_pair(signals_remove_right[j], signals_remove_right[j + 1], increasing) {
                            is_safe_after_remove_right = false;
                            break;
                        }
                    }

                    if !is_safe_after_remove_left && !is_safe_after_remove_right {
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

