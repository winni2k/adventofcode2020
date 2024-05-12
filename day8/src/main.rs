use std::{fs, iter::zip};


fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut command = vec!();
    let mut offset: Vec<i32> = vec!();
    for line in contents.lines() {
        let words: Vec<_> = line.split(' ').collect();
        command.push(words[0]);
        offset.push(words[1].parse().unwrap());
    }
    for (com, off) in zip(&command, &offset) {
        println!("{} {}", com, off);
    }

    let mut acc: i32 = 0;
    let mut i = 0;
    let mut seen = vec!();
    while i != command.len() {
        if seen.contains(&i) {
            panic!("Found instruction run a second time. i = {}, acc = {}", i, acc);
        }
        seen.push(i);
        match command[i] {
            "acc" => {
                acc += offset[i];
                i += 1;
            },
            "nop" => i += 1,
            "jmp" => i = (i as i32 + offset[i]) as usize,
            _ => panic!("Unexpected operator"),
        }
    }
}
