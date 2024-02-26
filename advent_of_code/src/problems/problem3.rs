const TRIANGLE_SIZE: usize = 3;

pub fn solve(data: &str) {
    let mut num_valid_triangles = 0;
    let mut data_iter = data.split('\n');
    loop {
        let lines = vec![data_iter.next(), data_iter.next(), data_iter.next()];

        if lines.contains(&None) {
            break;
        }
        num_valid_triangles += get_valid_triangles(
            lines
                .iter()
                .map(|x| convert_line_to_values(x.unwrap()))
                .collect(),
        );
    }
    println!("Num valid Triangles: {num_valid_triangles}");
}

fn get_valid_triangles(elements: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in 0..TRIANGLE_SIZE {
        sum += is_triangle_valid([elements[0][i], elements[1][i], elements[2][i]]) as i32
    }
    sum
}

fn is_triangle_valid(elements: [i32; TRIANGLE_SIZE]) -> bool {
    let mut min_sum = 0;
    let mut max_value = 0;
    for element in elements {
        if element <= max_value {
            min_sum += element;
        } else {
            min_sum += max_value;
            max_value = element;
        }
    }
    min_sum > max_value
}

fn convert_line_to_values(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
