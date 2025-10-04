pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn vectors_same_direction(capability: &(i32, i32), move_vec: &(i32, i32)) -> bool {
    // Tolerance for floating-point errors.
    let tolerance: f64 = 1e-9;

    // Get magnitude of capability vector.
    let norm_capability: f64 =
        <i32 as Into<f64>>::into(capability.0.pow(2) + capability.1.pow(2)).sqrt();

    // Get magnitude of move vec.
    let norm_move: f64 = <i32 as Into<f64>>::into(move_vec.0.pow(2) + move_vec.1.pow(2)).sqrt();

    // Take dot product of both vectors.
    let dot_product = (capability.0 * move_vec.0) + (capability.1 * move_vec.1);

    // Compute cos of angle between vectors.
    let cos_theta = dot_product as f64 / (norm_capability * norm_move);

    // if cos theta - 1 is less than floating point error tolerance, Vectors point in same dir.
    (cos_theta - 1.0).abs() <= tolerance
}
