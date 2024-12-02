use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut first_list, mut second_list): (Vec<i32>, Vec<i32>) = reader.lines()
        .map(|line| {
            let elements: Vec<i32> = line.unwrap().split_whitespace()
                .map(|word| {
                    word.parse::<i32>().unwrap()
                })
                .collect();

            (elements[0], elements[1])
        })
        .unzip();

    first_list.sort();
    second_list.sort();

    let result = zip(first_list, second_list)
        .fold(0, |acc, (first_element, second_element)| {
            let distance = second_element - first_element;

            if distance >= 0 {
                acc + distance
            } else {
                acc - distance
            }
        });

    println!("{}", result);
}

