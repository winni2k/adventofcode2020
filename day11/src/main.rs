use std::{fs, iter::zip};

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let mut layout: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut layout2 = layout.clone();
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
    }
    dbg!(to_string_vec(&layout2));
    let n_occupied: usize = layout2
        .iter()
        .map(|row| row.iter().filter(|char| **char == '#').count())
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
    let xi32 = x as i32;
    let yi32 = y as i32;
    let mut n_occupied: u32 = 0;
    for py in (yi32 - 1)..(yi32 + 2) {
        for px in (xi32 - 1)..(xi32 + 2) {
            if py == yi32 && px == xi32 {
                continue;
            }
            if (0..layout.len() as i32).contains(&py) && (0..layout[0].len() as i32).contains(&px) {
                // println!("py: {}, px {}", py, px);
                let pyu = py as usize;
                let pxu = px as usize;
                match layout[pyu][pxu] {
                    'L' => (),
                    '#' => n_occupied += 1,
                    '.' => (),
                    _ => panic!("Unexpected char encountered"),
                }
            }
        }
    }
    match layout[y][x] {
        'L' => match n_occupied {
            0 => '#',
            _ => 'L',
        },
        '#' => match n_occupied {
            v if v < 4 => '#',
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
