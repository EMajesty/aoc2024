use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/input1")?;
    let reader = BufReader::new(file);

    let mut result: i32 = 0;
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        let sum = left[i] - right[i];
        result += sum.abs();
    }

    println!("{}", result);

    Ok(())
}
