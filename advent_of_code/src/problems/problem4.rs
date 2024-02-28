pub fn solve(data: &str) {
    for line in data.split('\n').filter(|x| x.len() > 0) {
        decrpyt_room_name(line);
    }
}

fn decrpyt_room_name(line: &str) {
    const LOWER_CASE_ASCII_START: u32 = 97;
    const NUM_ALPHABET_LETTERS: u32 = 26;

    let (room_value, _) = line.split_at(line.find('[').unwrap());
    let (room_elements, sector_id_str) = room_value.split_at(room_value.rfind('-').unwrap());

    let sector_id = sector_id_str[1..].parse::<u32>().unwrap();
    let mut decrypted = String::new();

    for element in room_elements.chars().filter(|x| x != &'-') {
        let ascii_val = element.to_ascii_lowercase() as u32 - LOWER_CASE_ASCII_START;
        let rotated_val = ((ascii_val + sector_id) % NUM_ALPHABET_LETTERS) + LOWER_CASE_ASCII_START;
        decrypted.push(char::from_u32(rotated_val).unwrap());
    }
    if decrypted == "northpoleobjectstorage" {
        println!("Decrypted Room Name: {decrypted}, Sector Id: {sector_id}");
    }
}
