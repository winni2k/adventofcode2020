use std::{fs, process::exit};

fn main() {
    let path = "./input.txt";
    let window_size = 25;
    let contents = fs::read_to_string(path).unwrap();

    let numbers: Vec<_> = contents
        .lines()
        .map(|word| word.parse::<u64>().unwrap())
        .collect();

    let mut window = [0, window_size];
    let mut target = 0;
    while window[1] + 1 != numbers.len() {
        target = window[1];
        let mut valid = false;
        'outer: for i in window[0]..window[1] {
            for j in (window[0] + 1)..window[1] {
                if numbers[i] + numbers[j] == numbers[target] {
                    valid = true;
                    // println!(
                    //     "Hit: [{}]={}, [{}]={}, [{}]={}, window: {:?}",
                    //     i, numbers[i], j, numbers[j], target, numbers[target], window,
                    // );
                    break 'outer;
                }
            }
        }
        if !valid {
            println!("First invalid num: {}", numbers[target]);
            break;
        }
        window[0] += 1;
        window[1] += 1;
    }
    let invalid_num = numbers[target];
    for start in 0..(numbers.len() - 1) {
        let mut running_tally = 0;
        for end in (start + 1)..numbers.len() {
            running_tally += numbers[end - 1];
            match running_tally {
                v if v == invalid_num => {
                    let n_min = numbers[start..end].iter().min().unwrap();
                    let n_max = numbers[start..end].iter().max().unwrap();
                    println!("min, max, sum: {}, {}, {}", n_min, n_max, n_min + n_max);
                    exit(0);
                }
                v if v > invalid_num => {
                    break;
                }
                _ => (),
            }
        }
    }
}
