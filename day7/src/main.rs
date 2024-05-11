use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut bags: HashMap<String, Vec<String>> = HashMap::new();
    for line in contents.lines() {
        let words: Vec<_> = line.split(' ').map(|word| word.trim_end_matches(&[',', '.'])).collect();
        let bag_type = format!("{} {}", words[0], words[1]);
        if words[4] == "no" {
            bags.insert(bag_type, vec!());
        } else {
            let mut target_bags = vec!();
            for (index, _) in words.iter().enumerate() {
                if index > 4 && index % 4 == 1 {
                    target_bags.push(format!("{} {}", words[index], words[index+1]));
                }
            }
            bags.insert(bag_type, target_bags);
        }
        dbg!(words);
    }
    dbg!(&bags);

    let golden_bag = "shiny gold";
    let mut bags_containing_golden = vec!();
    for key in bags.keys() {
        // let mut seen = vec!();
        let mut unseen = bags[key].clone();
        while let Some(target) = unseen.pop() {
            if target == golden_bag {
                bags_containing_golden.push(key.clone());
                break;
            }
            for bag in &bags[&target] {
                unseen.push(bag.clone());
            }
        }
    }
    println!("Bags containing shiny golden bags:");
    for bag in &bags_containing_golden {
        println!("{}", bag);
    }
    println!("Total: {}", bags_containing_golden.len());


}
