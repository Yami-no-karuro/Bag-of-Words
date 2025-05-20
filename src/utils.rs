use std::collections::HashMap;

pub fn cosine_similarity(vec_a: &HashMap<String, f64>, vec_b: &HashMap<String, f64>) -> f64 {
    let mut dot_product: f64 = 0.0;
    for (token, value_a) in vec_a.iter() {
        if let Some(value_b) = vec_b.get(token) {
            dot_product += value_a * value_b;
        }
    }

    let mut sum_squares_a: f64 = 0.0;
    for value in vec_a.values() {
        sum_squares_a += value * value;
    }

    let mut sum_squares_b: f64 = 0.0;
    for value in vec_b.values() {
        sum_squares_b += value * value;
    }

    let norm_a = sum_squares_a.sqrt();
    let norm_b = sum_squares_b.sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    return dot_product / (norm_a * norm_b);
}

