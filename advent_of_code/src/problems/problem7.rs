pub fn solve(data: &str) {
    let mut num_support_ssl = 0;
    for line in data.split('\n') {
        num_support_ssl += does_ip_support_ssl(line) as u32;
    }
    println!("Num IPs supporting SSL: {num_support_ssl}");
}

fn does_ip_support_ssl(ip_str: &str) -> bool {
    let mut valid = false;
    let sections: Vec<&str> = ip_str.split(['[', ']']).filter(|x| x.len() > 0).collect();
    let mut bab_sequences: Vec<String> = Vec::new();
    for i in (0..sections.len()).step_by(2) {
        bab_sequences.append(&mut get_bab_sequences(sections[i]));
    }
    for i in (1..sections.len()).step_by(2) {
        if does_contain_sequence(sections[i], &bab_sequences) {
            valid = true;
            break;
        }
    }
    valid
}

fn get_bab_sequences(section: &str) -> Vec<String> {
    let mut sequences: Vec<String> = Vec::new();
    let section_chars: Vec<char> = section.chars().collect();
    for i in 0..section_chars.len() - 2 {
        if section_chars[i] == section_chars[i + 2] && section_chars[i] != section_chars[i + 1] {
            sequences.push(format!(
                "{}{}{}",
                section_chars[i + 1],
                section_chars[i],
                section_chars[i + 1]
            ));
        }
    }
    sequences
}

fn does_contain_sequence(section: &str, sequences: &Vec<String>) -> bool {
    for sequence in sequences {
        if section.contains(sequence) {
            return true;
        }
    }
    false
}
