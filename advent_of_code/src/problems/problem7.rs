pub fn solve(data: &str) {
    let mut num_support_tls = 0;
    for line in data.split('\n') {
        num_support_tls += does_ip_support_tls(line) as u32;
    }
    println!("Num IPs supporting TLS: {num_support_tls}");
}

fn does_ip_support_tls(ip_str: &str) -> bool {
    let mut valid = false;
    for (index, section) in ip_str.split(['[', ']']).filter(|x| x.len() > 0).enumerate() {
        if !valid || index % 2 == 1 {
            let contains_abba = does_section_contain_abba(section);
            if contains_abba && index % 2 == 1 {
                valid = false;
                break;
            } else if contains_abba {
                valid = true;
            }
        }
    }
    valid
}

fn does_section_contain_abba(section: &str) -> bool {
    let section_chars: Vec<char> = section.chars().collect();
    for i in 0..section_chars.len() - 3 {
        if section_chars[i] == section_chars[i + 3]
            && section_chars[i + 1] == section_chars[i + 2]
            && section_chars[i] != section_chars[i + 1]
        {
            return true;
        }
    }
    false
}
