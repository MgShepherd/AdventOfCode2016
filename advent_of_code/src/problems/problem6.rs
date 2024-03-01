use std::collections::HashMap;

const MESSAGE_LENGTH: usize = 8;

pub fn solve(data: &str) {
    let mut letter_frequencies: Vec<HashMap<char, usize>> = vec![HashMap::new(); MESSAGE_LENGTH];
    for line in data.split('\n') {
        for (pos, element) in line.chars().enumerate() {
            let current_element = letter_frequencies[pos].entry(element).or_insert(0);
            *current_element += 1;
        }
    }

    let mut decoded_message = String::new();
    for frequency_map in letter_frequencies {
        decoded_message.push(
            *frequency_map
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _v)| k)
                .unwrap(),
        )
    }
    println!("The decoded message is: {decoded_message}");
}
