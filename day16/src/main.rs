use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    rc::Rc,
};

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let mut state = "classes";
    let mut bound_id_to_field_name = vec![];
    let mut left_bounds = BTreeMap::new();
    let mut right_bounds = BTreeMap::new();
    let mut my_ticket = vec![];
    let mut tickets = vec![];
    let mut bound_id = 0u32;
    for line in contents.lines() {
        match state {
            "classes" => {
                if line.is_empty() {
                    state = "your-ticket";
                    continue;
                }
                let line_sections = line.split(':').collect::<Vec<&str>>();
                let words = line_sections[1]
                    .trim_start()
                    .split(' ')
                    .collect::<Vec<&str>>();
                let field = Rc::new(line_sections[0]);
                for i in [0, 2] {
                    bound_id_to_field_name.push(field.clone());
                    let bounds = words[i].split('-').collect::<Vec<&str>>();
                    left_bounds
                        .entry(bounds[0].parse::<u32>().unwrap())
                        .and_modify(|curr: &mut Vec<u32>| curr.push(bound_id))
                        .or_insert(vec![bound_id]);
                    right_bounds
                        .entry(bounds[1].parse::<u32>().unwrap())
                        .and_modify(|curr: &mut Vec<u32>| curr.push(bound_id))
                        .or_insert(vec![bound_id]);
                    bound_id += 1;
                }
            }
            "your-ticket" => {
                if line.is_empty() {
                    state = "other-tickets";
                    continue;
                }
                if line.starts_with("your ticket") {
                    continue;
                }
                my_ticket = line
                    .split(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            }
            "other-tickets" => {
                if line != "nearby tickets:" {
                    tickets.push(
                        line.split(',')
                            .map(|v| v.parse::<u32>().unwrap())
                            .collect::<Vec<u32>>(),
                    );
                }
            }
            _ => panic!(),
        }
    }
    let mut invalid_tickets = vec![];
    for (ticket_idx, ticket) in tickets.iter().enumerate() {
        for num in ticket {
            let candidates = get_candidate_bound_ids(&left_bounds, &right_bounds, *num);
            if candidates.is_empty() {
                invalid_tickets.push(ticket_idx);
            }
        }
    }
    for &ticket_idx in invalid_tickets.iter().rev() {
        tickets.swap_remove(ticket_idx);
    }
    println!("Invalid tickets: {:?}", invalid_tickets);
    println!("Valid tickets: {:?}", tickets);
    dbg!(&bound_id_to_field_name);

    let mut ticket_field_names: Vec<Rc<&str>> = vec![];
    let mut field_name_candidates = vec![];
    for ticket_num_idx in 0..tickets[0].len() {
        let mut candidate_fields = vec![];
        for ticket in &tickets {
            // dbg!(&ticket[ticket_num_idx]);
            candidate_fields.push(
                get_candidate_bound_ids(&left_bounds, &right_bounds, ticket[ticket_num_idx])
                    .iter()
                    // .inspect(|bound_id| println!("bound_id: {bound_id}"))
                    .map(|&bound_id| bound_id_to_field_name[bound_id as usize].clone())
                    .collect::<HashSet<Rc<&str>>>(),
            )
        }
        // dbg!(&candidate_fields);
        let mut intersection = candidate_fields[0].clone();
        for c_fields in &candidate_fields[1..candidate_fields.len()] {
            intersection = intersection.intersection(c_fields).cloned().collect();
        }
        // dbg!(&intersection, &ticket_num_idx);

        field_name_candidates.push((ticket_num_idx, intersection));
    }
    field_name_candidates.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    dbg!(&field_name_candidates);
    for i in 1..field_name_candidates.len() {
        let (source, to_be_changed) = field_name_candidates.split_at_mut(i);
        let candidates = &source[i - 1].1;
        assert!(candidates.len() == 1);
        let candidate = candidates.iter().next().unwrap();
        for field_name_candidate in to_be_changed.iter_mut() {
            field_name_candidate.1.remove(candidate);
        }
    }
    dbg!(&field_name_candidates);
    let mut field_names = field_name_candidates
        .iter()
        .map(|(idx, candidate_set)| (candidate_set.iter().next().unwrap().clone(), *idx))
        .collect::<Vec<(Rc<&str>, usize)>>();
    dbg!(&field_names, &my_ticket);
    let mut result = 1u64;
    for (field_name, field_id) in field_names {
        if field_name.starts_with("departure") {
            dbg!(&result, &field_id);
            result *= my_ticket[field_id] as u64;
        }
    }
    println!("Result: {result}");
}

fn get_candidate_bound_ids(
    left_bounds: &BTreeMap<u32, Vec<u32>>,
    right_bounds: &BTreeMap<u32, Vec<u32>>,
    num: u32,
) -> Vec<u32> {
    let mut candidates_left = HashSet::new();
    let mut candidates_right = HashSet::new();
    for (_, bound_ids) in left_bounds.range(..num + 1) {
        candidates_left.extend(bound_ids.iter().cloned());
    }
    for (_, bound_ids) in right_bounds.range(num..) {
        candidates_right.extend(bound_ids.iter().cloned());
    }
    candidates_left
        .intersection(&candidates_right)
        .cloned()
        .collect::<Vec<u32>>()
}
