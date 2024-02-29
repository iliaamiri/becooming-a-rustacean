use std::usize;

pub fn median(vector: &mut Vec<i32>) -> f32 {
    if vector.len() < 1 {
        return 0.0;
    }

    vector.sort();

    let vector_length = vector.len() as f32;

    let half_index = (vector_length / 2.0).floor() as usize;
    if vector_length % 2.0 == 0.0 {
       (vector[half_index - 1] + vector[half_index]) as f32 / 2.0
    } else {
        vector[half_index] as f32
    }
}
