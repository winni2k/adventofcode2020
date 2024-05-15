use std::{fs, iter::repeat_with, iter::zip};

fn main() {
    let path = "./input.txt";
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
    get_counter(&bus_id_offset_pairs_sorted)
}
fn get_counter(ids_offsets: &[(u64, u64)]) -> u64 {
    // dbg!(i, ids_offsets);
    // ((i * ids_offsets[0].0 - ids_offsets[1].1) as f32 / ids_offsets[1].0 as f32).ceil() as u64;
    // let increments = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut i = 0u64;
    let int_seq = repeat_with(|| {
        i += 1;
        i
    });
    int_seq
        // .par_bridge()
        .filter(|&x| {
            (ids_offsets[1].1 + ids_offsets[0].0 * x - ids_offsets[0].1) % ids_offsets[1].0 == 0
        })
        .map(|x| (x, ids_offsets[0].0 * x - ids_offsets[0].1))
        .find(|(j, counter)| {
            if j % 10000 == 0 {
                println!("{j}: {counter}");
            }
            for (bus_id, offset) in &ids_offsets[2..ids_offsets.len()] {
                if (counter + *offset) % bus_id != 0u64 {
                    return false;
                }
            }
            true
        })
        .unwrap()
        .1
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
