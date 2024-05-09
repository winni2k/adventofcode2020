use std::{fs};

fn main() { 
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut group_counts: Vec<i32> = Vec::new();
    let mut group_letters: Vec<char> = Vec::new();
    let mut first = true;
    for line in contents.lines() {
        if first {
            add_letters(line, &mut group_letters);
            first = false;
            continue;
        }
        if line.is_empty() {
            count_group_letters(&mut group_counts, &mut group_letters);
            first = true;
            continue;
        }
        remove_letters(line, &mut group_letters);
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

fn remove_letters(line: &str, group_letters: &mut Vec<char>) {
    let mut letters_to_remove: Vec<usize> = Vec::new();
    for (index, letter) in group_letters.iter().enumerate() {
        if !line.contains(&letter.to_string()) {
            letters_to_remove.push(index);
        }
    }
    letters_to_remove.sort();
    for index in letters_to_remove.iter().rev() {
        group_letters.swap_remove(*index);
    }
}