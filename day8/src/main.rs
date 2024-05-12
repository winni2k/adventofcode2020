use std::{fs, iter::zip};

#[derive(Debug)]
enum MyError {
    InfiniteLoop(),
}


fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut command = vec![];
    let mut offset: Vec<i32> = vec![];
    for line in contents.lines() {
        let words: Vec<_> = line.split(' ').collect();
        command.push(words[0]);
        offset.push(words[1].parse().unwrap());
    }
    for (com, off) in zip(&command, &offset) {
        println!("{} {}", com, off);
    }

    for i in 0..command.len(){
        let old_op = command[i];
        match old_op {
            "nop" => command[i] = "jmp" ,
            "jmp" => command[i] = "nop",
            "acc" => continue,
            _ => panic!("Unexpected operator"),

        }
        if let Ok(acc) = run_code(&command, &offset) {
            println!("Successful run. acc = {}", acc);
            break;
        }
        command[i] = old_op;
    } 
}

fn run_code(command: &[&str], offset: &[i32]) -> Result<i32, MyError>{
    let mut acc: i32 = 0;
    let mut i = 0;
    let mut seen = vec![];
    while i != command.len() {
        if seen.contains(&i) {
            return Err(
                MyError::InfiniteLoop(
            ));
        }
        seen.push(i);
        match command[i] {
            "acc" => {
                acc += offset[i];
                i += 1;
            }
            "nop" => i += 1,
            "jmp" => i = (i as i32 + offset[i]) as usize,
            _ => panic!("Unexpected operator"),
        }
    }
    Ok(acc)
}
