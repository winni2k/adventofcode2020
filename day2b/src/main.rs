use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut n_valid_pw: u32 = 0;
    for cap in re.captures_iter(&contents) {
        let first = cap[1].parse::<u32>().unwrap() as usize;
        let second = cap[2].parse::<u32>().unwrap() as usize;
        if (cap[4][first-1..first] == cap[3]) != (cap[4][second-1..second] == cap[3]){
            println!("{}-{} {}: {}", &cap[1], &cap[2], &cap[3], &cap[4]);
            n_valid_pw += 1;
        }
    }
    println!("n valid passwords: {}", n_valid_pw);
}
