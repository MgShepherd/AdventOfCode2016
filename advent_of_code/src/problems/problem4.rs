use std::cmp::Ordering;
use std::collections::HashMap;

const NUM_MAX_ELEMENTS: usize = 5;

pub fn solve(data: &str) {
    let mut real_id_sum = 0;
    for line in data.split('\n').filter(|x| x.len() > 0) {
        real_id_sum += get_room_value(line);
    }
    println!("Real Room ID Sum: {real_id_sum}");
}

fn get_room_value(line: &str) -> i32 {
    let (room_value, checksum) = line.split_at(line.find('[').unwrap());
    let room_elements: Vec<&str> = room_value.split("-").collect();
    let mut character_counts: HashMap<char, u32> = HashMap::new();

    for i in 0..(room_elements.len() - 1) {
        room_elements[i].chars().for_each(|c| {
            match character_counts.get(&c) {
                Some(count) => character_counts.insert(c, count + 1),
                None => character_counts.insert(c, 1),
            };
        })
    }
    let mut ordered_elements: Vec<(&char, &u32)> = character_counts.iter().collect();
    let checksum_chars: Vec<char> = checksum.chars().collect();
    ordered_elements.sort_by(|a, b| {
        let val_compare = b.1.cmp(&a.1);
        if val_compare == Ordering::Equal {
            return a.0.cmp(&b.0);
        }
        val_compare
    });
    for i in 0..NUM_MAX_ELEMENTS {
        if &checksum_chars[i + 1] != ordered_elements[i].0 {
            return 0;
        }
    }
    room_elements[room_elements.len() - 1]
        .parse::<i32>()
        .unwrap()
}
