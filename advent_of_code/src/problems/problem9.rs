pub fn solve(data: &str) {
    let mut i = 0;
    let mut decompressed_len = 0;
    let elements: Vec<char> = data.trim().chars().collect();
    while i < elements.len() {
        match elements[i] {
            '(' => decompressed_len += get_decompressed_sequence_length(&data[(i + 1)..], &mut i),
            _ => decompressed_len += 1,
        }
        i += 1;
    }
    println!("Decompressed Length of file: {decompressed_len}");
}

fn get_decompressed_sequence_length(elements: &str, current_index: &mut usize) -> usize {
    let (sequence, _) = elements.split_at(elements.find(')').unwrap());
    let sequence_information: Vec<usize> = sequence
        .split('x')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut decompressed_length = 0;
    let start_location = elements.find(')').unwrap() + 1;
    let mut i = 0;
    while i < sequence_information[0] {
        match elements.chars().nth(start_location + i).unwrap() {
            '(' => {
                decompressed_length +=
                    get_decompressed_sequence_length(&elements[(start_location + i + 1)..], &mut i)
                        * sequence_information[1];
            }
            _ => decompressed_length += sequence_information[1],
        }
        i += 1;
    }
    *current_index += start_location + sequence_information[0];

    decompressed_length
}
