use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let (first_list, second_list): (Vec<i32>, Vec<i32>) = reader.lines()
        .map(|line| {
            let elements: Vec<i32> = line.unwrap().split_whitespace()
                .map(|word| word.parse::<i32>().unwrap())
                .collect();

            (elements[0], elements[1])
        })
        .unzip();

    let mut tested_ids = HashMap::<i32, i32>::new();

    let similarity_score = first_list.into_iter().fold(0, |acc, elem| {
        let times: i32 = match tested_ids.get(&elem) {
            Some(times) => *times,
            None => {
                let mut times = 0i32;
                for second_elem in &second_list {
                    if *second_elem == elem {
                        times += 1;
                    }
                }

                tested_ids.insert(elem, times);

                times
            }
        };

        acc + elem * times
    });

    println!("{}", similarity_score);
}

