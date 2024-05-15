use std::fs;

fn main() {
    let path = "./input.txt";
    let contents = fs::read_to_string(path).unwrap();

    let instructions: Vec<(char, i32)> = contents
        .lines()
        .map(|line| {
            (
                line.chars().next().unwrap(),
                line[1..line.len()].parse().unwrap(),
            )
        })
        .collect();
    dbg!(&instructions);

    let mut position = [0, 0]; // [x, y]
    let mut waypoint = [10, 1];

    for (instruction, distance) in instructions {
        dbg!(&waypoint, &position);
        dbg!(instruction, distance);
        match instruction {
            'L' => adjust_waypoint(&mut waypoint, distance as f32),
            'R' => adjust_waypoint(&mut waypoint, -distance as f32),
            'N' => waypoint[1] += distance,
            'S' => waypoint[1] -= distance,
            'E' => waypoint[0] += distance,
            'W' => waypoint[0] -= distance,
            'F' => {
                position[0] += waypoint[0] * distance;
                position[1] += waypoint[1] * distance;
            }
            _ => panic!("Unexpected instruction"),
        };
    }
    dbg!(&waypoint, &position);
    println!(
        "Manhattan distance: {}",
        position[0].abs() + position[1].abs()
    );
}

fn adjust_heading(heading_rad: f32, angle: f32) -> f32 {
    let pi = std::f32::consts::PI;
    let angle_rad = 2f32 * pi * angle / 360f32;
    heading_rad + angle_rad
}

fn adjust_waypoint(waypoint: &mut [i32; 2], angle: f32) {
    dbg!(&waypoint);
    let pi = std::f32::consts::PI;
    let angle_rad = 2f32 * pi * angle / 360f32;
    dbg!(angle_rad);
    let hyp = ((waypoint[0].pow(2) + waypoint[1].pow(2)) as f32).sqrt();
    dbg!(hyp);
    let adj = waypoint[0] as f32 / hyp;
    let opp = waypoint[1] as f32 / hyp;
    let heading_rad = match opp.is_sign_positive() {
        true => adj.acos(),
        false => -adj.acos(),
        _ => panic!("Unexpected operator val"),
    };
    dbg!(heading_rad);
    let new_heading_rad = heading_rad + angle_rad;
    dbg!(new_heading_rad);
    waypoint[0] = (new_heading_rad.cos() * hyp).round() as i32;
    waypoint[1] = (new_heading_rad.sin() * hyp).round() as i32;
    dbg!(&waypoint);
}

#[cfg(test)]
mod tests {
    use super::*; // Import all items from the outer module

    #[test]
    fn test_adjust_waypoint() {
        // given
        let mut waypoint = [-25, -22];
        let angle = 90f32;

        // when
        adjust_waypoint(&mut waypoint, angle);

        // then
        assert_eq!(waypoint, [22, -25]);
    }
}
