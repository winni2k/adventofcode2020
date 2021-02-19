use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let trees: Vec<Vec<char>> = contents
        .strip_suffix("\n")
        .unwrap()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();

    let sled_params = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut n_trees: Vec<u32> = Vec::new();
    for params in sled_params.iter() {
        n_trees.push(count_trees_hit(&trees, params[1], params[0]));
    }

    let tree_product = n_trees.iter().fold(1.0, |acc, x| acc * *x as f64);
    println!("N trees hit = {:?} {:?}", n_trees, tree_product);
}

fn count_trees_hit(trees: &Vec<Vec<char>>, row_stride: usize, col_stride: usize) -> u32 {
    let mut col: usize = 0;
    let mut n_trees: u32 = 0;
    let mut i: usize = 0;
    while i < trees.len() {
        let row = &trees[i];
        if row[col] == '#' {
            n_trees += 1;
        }
        col = (&col + col_stride) % row.len();
        i += row_stride;
    }
    n_trees
}
