use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::iter::zip;

struct Bag {
    name: String,
    contains: Vec<String>,
    num_bags: Vec<i64>,
    total_bags: Option<i64>,
}

impl Bag {
    fn new(string_definition: &str) -> Bag {
        let words: Vec<_> = string_definition
            .split(' ')
            .map(|word| word.trim_end_matches(&[',', '.']))
            .collect();
        let name = format!("{} {}", words[0], words[1]);
        let mut contains = vec![];
        let mut num_bags: Vec<i64> = vec![];

        if words[4] != "no" {
            for (index, _) in words.iter().enumerate() {
                if index > 4 && index % 4 == 1 {
                    contains.push(format!("{} {}", words[index], words[index + 1]));
                    num_bags.push(words[index - 1].parse().unwrap())
                }
            }
        }

        Bag {
            name,
            contains,
            num_bags,
            total_bags: None,
        }
    }
}

impl fmt::Debug for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bag {{ name: \"{}\", contains: {:?}, num_bags: {:?}, total_bags: {:?}}}",
            self.name, self.contains, self.num_bags, self.total_bags,
        )
    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut bags: HashMap<String, Bag> = HashMap::new();
    for line in contents.lines() {
        let bag = Bag::new(line);
        bags.insert(bag.name.clone(), bag);
    }
    dbg!(&bags);

    let golden_bag = "shiny gold";
    for key in bags.keys().cloned().collect::<Vec<_>>() {
        dbg!(&key);
        if bags[&key].total_bags.is_some() {
            continue;
        }
        let mut unseen = vec![key.clone()];
        unseen.extend(bags[&key].contains.clone());
        while let Some(target) = unseen.pop() {
            // dbg!(&unseen);
            // dbg!(&target);
            match &bags[&target].total_bags {
                Some(_) => {
                    continue;
                }
                None => {
                    // println!("None match");
                    if bags[&target].contains.is_empty() {
                        // println!("No contains");
                        bags.get_mut(&target).unwrap().total_bags = Some(1);
                    } else if bags[&target]
                        .contains
                        .iter()
                        .all(|bag_name| bags[bag_name].total_bags.is_some())
                    {
                        // println!("All contained bags have total bag counts");
                        let target_bag = &bags[&target];
                        let total_bags = zip(&target_bag.contains, &target_bag.num_bags)
                            .map(|(name, n)| bags[name].total_bags.unwrap() * n)
                            .sum::<i64>()
                            + 1;
                        bags.get_mut(&target).unwrap().total_bags = Some(total_bags);
                    } else {
                        // println!("Bump");
                        unseen.push(target.clone());
                        unseen.extend(bags[&target].contains.clone());
                    }
                }
            }
            // dbg!(&bags);
        }
    }
    for value in bags.values() {
        println!("{:?}", value);
    }
    println!("Shiny gold bag: {:?}", bags[golden_bag]);
    println!(
        "Shiny gold bag contains: {}",
        bags[golden_bag].total_bags.unwrap() - 1
    );
}
