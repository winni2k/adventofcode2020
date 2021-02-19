use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let trees: Vec<Vec<char>> = contents
        .strip_suffix("\n")
        .unwrap()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    let mut col: usize = 0;
    let mut n_trees: u32 = 0;
    for row in trees {
        if row[col] == '#' {
            n_trees += 1;
        }
        col = (&col + 3) % row.len();
    }
    println!("N trees hit = {}", n_trees);




}
