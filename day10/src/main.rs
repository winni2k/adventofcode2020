use std::{collections::HashMap, fs, iter::zip};

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut numbers: Vec<_> = contents
        .lines()
        .map(|word| word.parse::<i32>().unwrap())
        .collect();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.last().unwrap() + 3);

    let mut jolts = vec![];
    for (i, j) in zip(0..numbers.len() - 1, 1..numbers.len()) {
        let jolt = numbers[j] - numbers[i];
        if !(1..4).contains(&jolt) {
            panic!("Unexpected jolt: {}", jolt);
        }
        jolts.push(jolt);
    }
    let mut counts = HashMap::new();
    for jolt in jolts {
        *counts.entry(jolt).or_insert(0) += 1;
    }
    dbg!(&counts);
    println!("Result: {}", counts[&1] * counts[&3])
}
