use std::{collections::HashMap, fs};

fn main() {
    let path = "./test.txt";
    let contents = fs::read_to_string(path).unwrap();
    let input = contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|w| w.parse().unwrap())
        .collect::<Vec<i32>>();
    println!("Result: {}", get_nth_num(&input, 30000000));
}
fn get_nth_num(input: &Vec<i32>, n: usize) -> i32 {
    assert!(input.len() < n);
    let mut mem = HashMap::new();
    for (idx, val) in input.iter().enumerate() {
        mem.insert(*val, idx as i32 + 1);
    }
    let mut idx = input.len() as i32 + 1;
    let mut next_val = 0i32;
    let mut this_val = 0i32;
    while idx as usize != n + 1 {
        this_val = next_val;
        match mem.get(&next_val) {
            Some(i) => next_val = idx as i32 - *i,
            None => next_val = 0i32,
        }
        mem.insert(this_val, idx);

        idx += 1;
    }
    return this_val;
}
#[cfg(test)]
mod tests {
    use super::*; // Import all items from the outer module
    use test_case::test_case;

    #[test]
    fn test_example_1() {
        // given
        let input = &vec![0, 3, 6];

        // when / then
        assert_eq!(get_nth_num(input, 4), 0);
        assert_eq!(get_nth_num(input, 5), 3);
        assert_eq!(get_nth_num(input, 6), 3);
        assert_eq!(get_nth_num(input, 7), 1);
        assert_eq!(get_nth_num(input, 8), 0);
        assert_eq!(get_nth_num(input, 9), 4);
        assert_eq!(get_nth_num(input, 10), 0);
        assert_eq!(get_nth_num(input, 2020), 436);
    }

    #[test_case([1,3,2], 1)]
    #[test_case([2,1,3], 10)]
    #[test_case([1,2,3], 27)]
    #[test_case([2,3,1], 78)]
    #[test_case([3,2,1], 438 )]
    #[test_case([3,1,2], 1836 )]
    fn test_more_examples(input: [i32; 3], expected: i32) {
        assert_eq!(get_nth_num(&input.to_vec(), 2020), expected);
    }
}
