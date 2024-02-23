enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn solve(data: &str) {
    let mut current_direction = Direction::North;
    let mut current_point = Point { x: 0, y: 0 };

    for instruction in data.split(',').map(|x| x.trim()) {
        if instruction.chars().nth(0).unwrap() == 'L' {
            current_direction = turn_left(&current_direction);
        } else {
            current_direction = turn_right(&current_direction);
        }
        let move_amount = instruction[1..].parse::<i32>().unwrap();
        move_in_direction(&current_direction, &mut current_point, move_amount);
    }

    println!("Final Position: {:?}", current_point);
    println!(
        "Distance from origin: {}",
        get_distance_from_origin(&current_point)
    );
}

fn get_distance_from_origin(current_point: &Point) -> i32 {
    current_point.x.abs() + current_point.y.abs()
}

fn move_in_direction(current_direction: &Direction, current_point: &mut Point, move_amount: i32) {
    match current_direction {
        Direction::North => current_point.y -= move_amount,
        Direction::East => current_point.x += move_amount,
        Direction::South => current_point.y += move_amount,
        Direction::West => current_point.x -= move_amount,
    }
}

fn turn_left(current_direction: &Direction) -> Direction {
    match current_direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn turn_right(current_direction: &Direction) -> Direction {
    match current_direction {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}
