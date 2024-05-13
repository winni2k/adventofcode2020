use std::fs;

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let numbers: Vec<_> = contents
        .lines()
        .map(|word| word.parse::<u64>().unwrap())
        .collect();

    let mut window = [0, 25];
    while window[1] + 1 != numbers.len() {
        let target = window[1];
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
}
