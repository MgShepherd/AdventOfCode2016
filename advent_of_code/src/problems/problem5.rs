const PUZZLE_INPUT: &str = "ugkcyxxp";
const PASSWORD_LENGTH: usize = 8;
const NUM_LEADING_ZEROS: usize = 5;

pub fn solve() {
    let mut index = 0;
    let mut password_array = ['_'; PASSWORD_LENGTH];
    loop {
        if !password_array.contains(&'_') {
            break;
        }
        let hash = format!("{:?}", md5::compute(format!("{PUZZLE_INPUT}{index}")));
        if &hash[0..NUM_LEADING_ZEROS] == "00000" {
            let insert_index = hash
                .chars()
                .nth(NUM_LEADING_ZEROS)
                .unwrap()
                .to_digit(10)
                .unwrap_or(u32::MAX) as usize;
            if insert_index < PASSWORD_LENGTH && password_array[insert_index] == '_' {
                println!("{hash}");
                password_array[insert_index] = hash.chars().nth(NUM_LEADING_ZEROS + 1).unwrap();
            }
        }
        index += 1;
    }
    println!();
    for element in password_array {
        print!("{element}");
    }
    println!();
}
