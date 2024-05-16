use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Memory {
    mem: HashMap<u32, u64>,
    mask: String,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            mem: HashMap::new(),
            mask: String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
        }
    }
    fn fill_mask(&mut self, word: &str) {
        self.mask.clear();
        self.mask.push_str(word);
    }
    fn add_value(&mut self, location_str: &str, value_str: &str) {
        let location: u32 = location_str[4..(location_str.len() - 1)].parse().unwrap();
        let mut value = value_str.parse::<u64>().unwrap();
        // dbg!(&self.mask, value_str);
        for (idx, mask_char) in self.mask.chars().rev().enumerate() {
            // dbg!(idx, mask_char, char);
            // let value_bit = (value >> idx) & 1;
            match mask_char {
                'X' => (),
                '1' => value |= 1 << idx,
                '0' => value &= !(1 << idx),
                _ => panic!(),
            }
        }
        dbg!(location, value);
        self.mem.insert(location, value);
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
        assert_eq!(memory.mem[&7u32], 11u64);
    }
    #[test]
    fn test_add_to_memory_with_mask() {
        // given
        let mut memory = Memory::new();
        memory.fill_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        let location_str = "mem[7]";
        let value_str = "11";

        // when
        memory.add_value(location_str, value_str);

        // then
        assert_eq!(memory.mem[&7u32], 73u64);
    }
}
