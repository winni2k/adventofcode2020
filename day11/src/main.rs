use std::{fs, iter::zip};

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut layout: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut layout2 = layout.clone();
    let mut i = 0;
    loop {
        for i in 0..layout.len() {
            for j in 0..layout[0].len() {
                layout2[i][j] = calc_seat_state(&layout, j, i);
            }
        }

        dbg!(to_string_vec(&layout2));
        let n_changes = n_seats_changed(&layout, &layout2);
        dbg!(n_changes);
        if n_changes == 0 {
            break;
        }
        layout = layout2.clone();
        i += 1;
        // if i == 2 {
        //     break;
        // }
    }
    dbg!(to_string_vec(&layout2));
    let n_occupied: usize = layout2
        .iter()
        .map(|row| row.iter().filter(|&&char| char == '#').count())
        .sum();
    dbg!(n_occupied);
}

fn n_seats_changed(layout1: &Vec<Vec<char>>, layout2: &Vec<Vec<char>>) -> u32 {
    let mut n_changes = 0;
    for (row1, row2) in zip(layout1, layout2) {
        for (char1, char2) in zip(row1, row2) {
            if *char1 != *char2 {
                // println!("{} != {}", *char1, *char2);
                n_changes += 1;
            }
        }
    }
    n_changes
}

fn calc_seat_state(layout: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    // dbg!(y, x);
    let xi32 = x as i32;
    let yi32 = y as i32;
    let mut n_occupied: u32 = 0;
    // directions start at top left and go clockwise. Last direction is left.
    let mut directions_explored = [false; 8];
    let directions = [
        (-1i32, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];
    let mut dir_factor = 0;
    while !directions_explored.iter().all(|&d| d) {
        dir_factor += 1;
        // dbg!(dir_factor);
        // dbg!(&directions_explored);
        for (idx, (d_y, d_x)) in directions.iter().enumerate() {
            if directions_explored[idx] {
                continue;
            }
            let yf = y as i32 + d_y * dir_factor;
            let xf = x as i32 + d_x * dir_factor;
            // dbg!(yf, xf, layout.len(), layout[0].len());
            if yf < 0 || xf < 0 || yf >= layout.len() as i32 || xf >= layout[0].len() as i32 {
                directions_explored[idx] = true;
            } else {
                // println!("yf, yx: {}, {}", yf, xf);
                match layout[yf as usize][xf as usize] {
                    'L' => {
                        directions_explored[idx] = true;
                    }
                    '#' => {
                        directions_explored[idx] = true;
                        n_occupied += 1;
                    }
                    '.' => (),
                    _ => panic!("Unexpected char encountered"),
                }
            }
        }
        // dbg!(directions_explored);
    }
    match layout[y][x] {
        'L' => match n_occupied {
            0 => '#',
            _ => 'L',
        },
        '#' => match n_occupied {
            v if v < 5 => '#',
            _ => 'L',
        },
        '.' => '.',
        _ => panic!("Unexpected char encountered"),
    }
}

fn to_string_vec(layout: &Vec<Vec<char>>) -> Vec<String> {
    let mut string_layout: Vec<String> = vec![];
    for row in layout {
        string_layout.push(row.iter().cloned().collect());
    }
    string_layout
}
