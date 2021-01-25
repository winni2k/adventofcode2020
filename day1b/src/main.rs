use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let nums: Vec<_> = contents.split('\n').filter_map(|s| s.parse::<u32>().ok()).collect();
    for (idx, i) in nums.iter().enumerate(){
        for (idx2, j) in nums[idx+1..].iter().enumerate() {
            for k in nums[idx+idx2+2..].iter(){
                if i + j + k == 2020 {
                    println!("{}, {}: {}, {}: {}", idx, i, idx2, j, k);
                    println!("Result is {}", i * j * k);
                    process::exit(0);
                }

            }

        }
    }
}
