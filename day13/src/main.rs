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
    let mut i = -1i64;
    let int_seq = repeat_with(|| {
        i += 1;
        i
    });
    let ids: Vec<u64> = ids_offsets.iter().map(|(id, _)| *id).collect();
    let offsets: Vec<u64> = ids_offsets.iter().map(|(_, offset)| *offset).collect();
    // let cr = chinese_remainder([offsets[0], offsets[1]], [ids[0], ids[1]]);
    dbg!(&ids, &offsets);
    let cr = chinese_remainder([offsets[0], offsets[1]], [ids[0], ids[1]]);
    let id_mult = ids[0] * ids[1];
    println!("CR: {cr}, id_mult: {id_mult}");

    int_seq
        // .inspect(|x| println!("Seq: {x}"))
        .map(|x| x * id_mult as i64 - cr)
        // .inspect(|x| println!("Candidate: {x}"))
        .filter(|&x| x > 0)
        .map(|x| x as u64)
        .enumerate()
        .find(|(j, counter)| {
            if j % 100000000 == 0 {
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
fn extended_euclid(a: u64, b: u64) -> [i64; 2] {
    let mut s = [1i64, 0];
    let mut t = [0i64, 1];
    let mut r = [a, b];
    assert!(r[0] > r[1]);
    while r[1] != 0 {
        let q = r[0] / r[1];
        let qi = q as i64;
        let ri = r[0] - q * r[1];
        r[0] = r[1];
        r[1] = ri;

        let si = s[0] - s[1] * qi;
        s[0] = s[1];
        s[1] = si;

        let ti = t[0] - t[1] * qi;
        t[0] = t[1];
        t[1] = ti;
    }
    [s[0], t[0]]
}

fn chinese_remainder(a: [u64; 2], n: [u64; 2]) -> i64 {
    let bez = extended_euclid(n[0], n[1]);
    dbg!(&bez);
    a[0] as i64 * bez[1] * n[1] as i64 + a[1] as i64 * bez[0] * n[0] as i64
}
#[cfg(test)]
mod tests {
    use super::*; // Import all items from the outer module

    #[test]
    fn test_extended_euclid() {
        // given
        let a = 240;
        let b = 46;

        // when
        let bez = extended_euclid(a, b);

        // then
        assert_eq!(bez[0], -9);
        assert_eq!(bez[1], 47);
    }
    #[test]
    fn test_extended_euclid_2() {
        // given
        let a = 4;
        let b = 3;

        // when
        let bez = extended_euclid(a, b);

        // then
        assert_eq!(bez[0], 1);
        assert_eq!(bez[1], -1);
    }
    #[test]
    fn test_extended_euclid_3() {
        // given
        let a = 12;
        let b = 5;

        // when
        let bez = extended_euclid(a, b);

        // then
        assert_eq!(bez[0], -2);
        assert_eq!(bez[1], 5);
    }
    #[test]
    fn test_chinese_remainder() {
        // from https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(constructive_proof)
        // given
        let a = [3, 0];
        let n = [4, 3];

        // when
        let res = chinese_remainder(a, n);

        // then
        assert_eq!(res, -9);
    }
    #[test]
    fn test_chinese_remainder_3_4_5() {
        // from https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_(constructive_proof)
        // given
        let a = [3, 4];
        let n = [12, 5];

        // when
        let res = chinese_remainder(a, n);

        // then
        assert_eq!(res, -21);
    }
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
