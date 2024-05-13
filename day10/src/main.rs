use std::{collections::HashMap, fs, iter::zip};

fn main() {
    let path = "./test.txt";
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
    // dbg!(&counts);
    println!("Result: {}", counts[&1] * counts[&3]);

    let mut unseen: Vec<Vec<usize>> = vec![vec![]];
    let mut seen: Vec<Vec<usize>> = vec![];
    // dbg!(&numbers);
    // dbg!(&unseen);
    let mut dbg_count = 0;
    while !unseen.is_empty() {
        if let Some(arr) = unseen.pop() {
            let first = *arr.last().unwrap_or(&1) + 1;
            for i in first..(numbers.len() - 1) {
                // dbg!(&i);
                let mut prev_idx = i - 1;
                while arr.contains(&prev_idx) {
                    prev_idx -= 1;
                }
                let mut next_idx = i + 1;
                while arr.contains(&next_idx) {
                    next_idx += 1;
                }
                if numbers[next_idx] - numbers[prev_idx] < 4 {
                    let mut candidate = arr.clone();
                    candidate.push(i);
                    unseen.push(candidate);
                }
            }
            seen.push(arr);
        }
        // dbg!(&seen);
        // dbg!(&unseen);
    }
    // dbg!(&seen);
    // dbg!(&unseen);
    println!("Number of charging arrangements: {}", seen.len());
}
