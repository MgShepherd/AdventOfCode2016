use crate::utils::data_utils::IndexPoint;

pub fn solve(data: &str) {
    let mut current_point = IndexPoint { x: 0, y: 2 };
    let mut code = String::new();

    for line in data.split('\n').filter(|x| !x.is_empty()) {
        code.push(get_number(&mut current_point, line));
    }
    println!("Code is: {code}");
}

fn get_number(current_point: &mut IndexPoint, line: &str) -> char {
    const KEYPAD_SIZE: usize = 5;
    const KEYPAD: [[char; KEYPAD_SIZE]; KEYPAD_SIZE] = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];

    for element in line.chars() {
        if element == 'U'
            && current_point.y > 0
            && KEYPAD[current_point.y - 1][current_point.x] != ' '
        {
            current_point.y -= 1;
        } else if element == 'D'
            && current_point.y < KEYPAD_SIZE - 1
            && KEYPAD[current_point.y + 1][current_point.x] != ' '
        {
            current_point.y += 1;
        } else if element == 'L'
            && current_point.x > 0
            && KEYPAD[current_point.y][current_point.x - 1] != ' '
        {
            current_point.x -= 1;
        } else if element == 'R'
            && current_point.x < KEYPAD_SIZE - 1
            && KEYPAD[current_point.y][current_point.x + 1] != ' '
        {
            current_point.x += 1;
        }
    }
    KEYPAD[current_point.y][current_point.x]
}
