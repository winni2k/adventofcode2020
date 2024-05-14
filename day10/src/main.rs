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
    // dbg!(&counts);
    println!("Result: {}", counts[&1] * counts[&3]);

    dbg!(&numbers);

    let deletion_positions = find_del_pos(numbers);
    dbg!(&deletion_positions);

    let grouped_del_pos = group_del_pos(&deletion_positions);
    let group_sizes: Vec<usize> = grouped_del_pos.iter().map(|group| group.len()).collect();
    dbg!(&group_sizes);
    let mut total_arrangements = 1u64;
    for group_size in group_sizes {
        let n_group_arrangements = 2u64.pow(group_size as u32);
        let n_invalid_arrangements = match group_size {
            v if v < 3 => 0,
            v => {
                let v2 = v - 2;
                v2 * (v2 + 1) / 2
            }
        };
        total_arrangements *= n_group_arrangements - n_invalid_arrangements as u64;
    }
    println!("Num arrangements: {}", total_arrangements);
}

fn group_del_pos(del_pos: &Vec<usize>) -> Vec<Vec<i32>> {
    let mut groups = vec![];
    let mut last = -1;
    for pos_usize in del_pos {
        let pos = *pos_usize as i32;
        if pos - last != 1 {
            groups.push(vec![pos]);
        } else {
            groups.last_mut().unwrap().push(pos);
        }
        last = pos
    }
    groups
}

fn find_del_pos(numbers: Vec<i32>) -> Vec<usize> {
    let mut deletion_positions = vec![];
    for i in 1..(numbers.len() - 1) {
        let jolt_diff = numbers[i + 1] - numbers[i - 1];
        match jolt_diff {
            2 => deletion_positions.push(i),
            3 => panic!("Unexpected jolt diff 3"),
            v if v > 3 => (),
            v => panic!("Unexpected jolt diff {}", v),
        }
    }
    deletion_positions
}
