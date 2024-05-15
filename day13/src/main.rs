use std::fs;

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut line_iter = contents.lines();
    let timestamp: u32 = line_iter.next().unwrap().parse().unwrap();
    let bus_ids: Vec<u32> = line_iter
        .next()
        .unwrap()
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    let mut next_departure_times = vec![];
    for bus_id in &bus_ids {
        let mut next_departure = 0;
        while next_departure < timestamp {
            next_departure += bus_id;
        }
        next_departure_times.push(next_departure);
    }
    dbg!(&timestamp);
    dbg!(&bus_ids);

    let (bus_idx, departure_time) = next_departure_times
        .iter()
        .enumerate()
        .min_by_key(|&(_, val)| val)
        .unwrap();
    let waiting_time = departure_time - timestamp;
    println!("{} - {} = {}", departure_time, timestamp, waiting_time);
    println!(
        "{} * {} = {}",
        bus_ids[bus_idx],
        waiting_time,
        bus_ids[bus_idx] * waiting_time
    );
}
