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

    *current_index += elements.find(')').unwrap() + 1 + sequence_information[0];
    sequence_information[0] * sequence_information[1]
}
