use crate::utils::data_utils::Point;

pub fn solve(data: &str) {
    let mut current_point = Point { x: 1, y: 1 };
    let mut code = String::new();

    for line in data.split('\n').filter(|x| !x.is_empty()) {
        code.push(get_number(&mut current_point, line));
    }
    println!("Code is: {code}");
}

fn get_number(current_point: &mut Point, line: &str) -> char {
    const KEYPAD_SIZE: i32 = 3;
    const KEYPAD: [[char; KEYPAD_SIZE as usize]; KEYPAD_SIZE as usize] =
        [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    for element in line.chars() {
        if element == 'U' && current_point.y > 0 {
            current_point.y -= 1;
        } else if element == 'D' && current_point.y < KEYPAD_SIZE - 1 {
            current_point.y += 1;
        } else if element == 'L' && current_point.x > 0 {
            current_point.x -= 1;
        } else if element == 'R' && current_point.x < KEYPAD_SIZE - 1 {
            current_point.x += 1;
        }
    }
    KEYPAD[current_point.y as usize][current_point.x as usize]
}
