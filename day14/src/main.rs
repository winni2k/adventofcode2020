use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Memory {
    mem: HashMap<u64, u64>,
    mask: String,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            mem: HashMap::new(),
            mask: String::from("000000000000000000000000000000000000"),
        }
    }
    fn fill_mask(&mut self, word: &str) {
        self.mask.clear();
        self.mask.push_str(word);
    }
    fn add_value(&mut self, location_str: &str, value_str: &str) {
        let mut location: u64 = location_str[4..(location_str.len() - 1)].parse().unwrap();
        let value = value_str.parse::<u64>().unwrap();
        for (idx, char) in self.mask.chars().rev().enumerate() {
            if char == '1' {
                location |= 1 << idx
            }
        }
        let mut locations: Vec<u64> = vec![location];
        let mut locations2 = vec![];
        for (idx, char) in self.mask.chars().rev().enumerate() {
            if char == 'X' {
                let bitmask = 1 << idx;
                let bitmask2 = !bitmask;
                for loc in &locations {
                    locations2.push(loc | bitmask);
                    locations2.push(loc & bitmask2);
                }
                locations.clear();
                locations.append(&mut locations2);
            }
        }
        for loc in locations {
            self.mem.insert(loc, value);
        }
    }
}

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let mut memory = Memory::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        // dbg!(&words);
        match words[0] {
            "mask" => memory.fill_mask(words[2]),
            w if w.starts_with("mem") => memory.add_value(w, words[2]),
            _ => panic!("Unexpected word in input line"),
        }
    }
    // dbg!(&memory);
    println!("Total: {}", memory.mem.values().sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*; // Import all items from the outer module

    #[test]
    fn test_mask_gen() {
        // given
        let mut memory = Memory::new();
        let word = "X1X0";

        // when
        memory.fill_mask(&word);

        // then
        assert_eq!(memory.mask, "X1X0");
    }
    #[test]
    fn test_add_to_memory() {
        // given
        let mut memory = Memory::new();
        let location_str = "mem[7]";
        let value_str = "11";

        // when
        memory.add_value(location_str, value_str);

        // then
        assert_eq!(memory.mem[&7], 11u64);
    }
    #[test]
    fn test_add_to_memory_with_mask() {
        // given
        let mut memory = Memory::new();
        memory.fill_mask("00000000000000000000000000000000X00X");
        let location_str = "mem[4]";
        let value_str = "11";

        // when
        memory.add_value(location_str, value_str);

        // then
        let mut keys = memory.mem.keys().cloned().collect::<Vec<u64>>();
        keys.sort();
        assert_eq!(keys, [4u64, 5, 12, 13]);
    }
}
