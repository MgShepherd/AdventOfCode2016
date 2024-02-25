pub fn solve(data: &str) {
    let mut num_valid_triangles = 0;
    for line in data.split('\n') {
        if is_triangle_valid(line) {
            num_valid_triangles += 1;
        }
    }
    println!("Num valid Triangles: {num_valid_triangles}");
}

fn is_triangle_valid(triangle: &str) -> bool {
    let mut min_sum = 0;
    let mut max_value = 0;
    for element in triangle
        .split(' ')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
    {
        if element <= max_value {
            min_sum += element;
        } else {
            min_sum += max_value;
            max_value = element;
        }
    }
    min_sum > max_value
}
