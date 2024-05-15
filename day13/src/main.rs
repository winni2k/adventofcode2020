use std::{fs, iter::zip};

fn main() {
    let path = "./test2.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut line_iter = contents.lines();
    line_iter.next();
    let bus_ids_and_offsets: Vec<&str> = line_iter.next().unwrap().split(',').collect();
    let mut offsets = vec![];
    for (idx, &val) in bus_ids_and_offsets.iter().enumerate() {
        if val != "x" {
            offsets.push(idx);
        }
    }
    let bus_ids: Vec<u64> = bus_ids_and_offsets
        .iter()
        .filter(|&x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    assert!(offsets.len() == bus_ids.len());
    println!("{}", find_timestamp_match(&bus_ids, &offsets));
}

fn find_timestamp_match(bus_ids: &Vec<u64>, offsets: &Vec<usize>) -> u64 {
    let bus_id_offset_pairs: Vec<_> = zip(bus_ids, offsets)
        .map(|tup| (*tup.0, *tup.1 as u64))
        .collect();
    let mut bus_id_offset_pairs_sorted = bus_id_offset_pairs.clone();
    bus_id_offset_pairs_sorted.sort_by_key(|&pair| std::u64::MAX - pair.0);
    dbg!(&bus_id_offset_pairs_sorted);
    let mut counter = bus_id_offset_pairs_sorted[0].0 - bus_id_offset_pairs_sorted[0].1;
    loop {
        let mut matching_bus_ids = true;
        for (bus_id, offset) in &bus_id_offset_pairs_sorted {
            if (counter + *offset) % bus_id != 0u64 {
                // println!("Mismatch: ({} + {}) % {} != 0", counter, offset, bus_id);
                matching_bus_ids = false;
                break;
            }
        }
        if counter % 10000000 == 0 {
            println!("Counter: {}", counter);
        }
        if matching_bus_ids {
            return counter;
        }
        counter += bus_id_offset_pairs_sorted[0].0;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import all items from the outer module

    #[test]
    fn test_find_timestamp_match_simple() {
        // given
        let bus_ids = vec![2, 7, 5];
        let offsets = vec![0, 2, 4];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 26);
    }
    #[test]
    fn test_find_timestamp_match_1() {
        // given
        let bus_ids = vec![17, 13, 19];
        let offsets = vec![0, 2, 3];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 3417);
    }
    #[test]

    fn test_find_timestamp_match_full_test_file() {
        // given
        let bus_ids = vec![7, 13, 59, 31, 19];
        let offsets = vec![0, 1, 4, 6, 7];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 1068781);
    }
    #[test]

    fn test_find_timestamp_match_large() {
        // given
        let bus_ids = vec![1789, 37, 47, 1889];
        let offsets = vec![0, 1, 2, 3];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 1202161486);
    }
    #[test]

    fn test_find_timestamp_match_2() {
        // given
        let bus_ids = vec![67, 7, 59, 61];
        let offsets = vec![0, 1, 2, 3];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 754018);
    }
    #[test]

    fn test_find_timestamp_match_3() {
        // given
        let bus_ids = vec![67, 7, 59, 61];
        let offsets = vec![0, 2, 3, 4];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 779210);
    }
    #[test]

    fn test_find_timestamp_match_4() {
        // given
        let bus_ids = vec![67, 7, 59, 61];
        let offsets = vec![0, 1, 3, 4];

        // when
        let timestamp = find_timestamp_match(&bus_ids, &offsets);

        // then
        assert_eq!(timestamp, 1261476);
    }
}
