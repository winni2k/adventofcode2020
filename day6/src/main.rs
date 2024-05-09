use std::{fs};

fn main() { 
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut group_counts: Vec<i32> = Vec::new();
    let mut group_letters: Vec<char> = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            count_group_letters(&mut group_counts, &mut group_letters);
        } else {
            add_letters(line, &mut group_letters);
        }
    }
    count_group_letters(&mut group_counts, &mut group_letters);
    let sum: i32 = group_counts.iter().sum();
    println!("Total group counts: {}", sum); 
}

fn count_group_letters(group_counts: &mut Vec<i32>, group_letters: &mut Vec<char>) {
    group_counts.push(group_letters.len().try_into().unwrap());
    group_letters.clear();
}

fn add_letters(line: &str, group_letters: &mut Vec<char>) {
    for letter in line.chars() {
        if !group_letters.contains(&letter){
            group_letters.push(letter);
        }
    }
}
