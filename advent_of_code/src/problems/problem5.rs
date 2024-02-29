const PUZZLE_INPUT: &str = "ugkcyxxp";
const NUM_LEADING_ZEROS: usize = 5;

pub fn solve() {
    let mut index = 0;
    let mut password = String::new();
    loop {
        if password.len() == 8 {
            break;
        }
        let hash = format!("{:?}", md5::compute(format!("{PUZZLE_INPUT}{index}")));
        if &hash[0..NUM_LEADING_ZEROS] == "00000" {
            password.push(hash.chars().nth(NUM_LEADING_ZEROS).unwrap());
        }
        index += 1;
    }
    println!("The Door Password is: {password}");
}
