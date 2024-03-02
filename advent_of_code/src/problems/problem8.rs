const NUM_ROWS: usize = 6;
const NUM_COLS: usize = 50;

pub fn solve(data: &str) {
    let mut display = vec![vec!['.'; NUM_COLS]; NUM_ROWS];
    for line in data.split('\n').filter(|x| x.len() > 0) {
        process_line(line, &mut display);
    }
    for row in &display {
        for element in row {
            print!("{element}");
        }
        println!();
    }

    println!("Number of lit pixels: {}", get_num_lit_pixels(&display));
}

fn process_line(line: &str, display: &mut Vec<Vec<char>>) {
    let components: Vec<&str> = line.split(' ').filter(|x| x.len() > 0).collect();
    match components[0] {
        "rect" => process_rect_command(&components, display),
        _ => process_rotate_command(&components, display),
    }
}

fn process_rect_command(components: &Vec<&str>, display: &mut Vec<Vec<char>>) {
    let dimensions: Vec<usize> = components[1]
        .split('x')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    for y in 0..dimensions[1] {
        for x in 0..dimensions[0] {
            display[y][x] = '#';
        }
    }
}

fn process_rotate_command(components: &Vec<&str>, display: &mut Vec<Vec<char>>) {
    let location_elements: Vec<usize> = components[2]
        .split('=')
        .filter(|x| x.parse::<usize>().is_ok())
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let amount = components[4].parse::<usize>().unwrap();
    match components[1] {
        "column" => rotate_column(location_elements[0], amount, display),
        _ => rotate_row(location_elements[0], amount, display),
    }
}

fn rotate_column(location: usize, amount: usize, display: &mut Vec<Vec<char>>) {
    let mut new_column: Vec<char> = vec!['.'; NUM_ROWS];
    for y in 0..NUM_ROWS {
        if display[y][location] == '#' {
            new_column[(y + amount) % NUM_ROWS] = '#';
        }
    }
    for y in 0..NUM_ROWS {
        display[y][location] = new_column[y];
    }
}

fn rotate_row(location: usize, amount: usize, display: &mut Vec<Vec<char>>) {
    let mut new_row: Vec<char> = vec!['.'; NUM_COLS];
    for x in 0..NUM_COLS {
        if display[location][x] == '#' {
            new_row[(x + amount) % NUM_COLS] = '#';
        }
    }
    display[location] = new_row;
}

fn get_num_lit_pixels(display: &Vec<Vec<char>>) -> usize {
    let mut num_lit = 0;
    for y in 0..NUM_ROWS {
        for x in 0..NUM_COLS {
            if display[y][x] == '#' {
                num_lit += 1;
            }
        }
    }
    num_lit
}
