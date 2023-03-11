pub fn is_square(num: usize) -> bool {
    let num_f32 = num as f32;
    let sqrt = num_f32.sqrt();
    return sqrt == sqrt.floor();
}

pub fn is_square_matrix<T>(data: &Vec<Vec<T>>) -> bool {
    let len: usize = data.len();

    data.iter().map(|v| v.len()).all(|l| l == len)
}